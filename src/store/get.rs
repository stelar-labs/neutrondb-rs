use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::error::Error;

use crate::Store;

impl<K,V> Store<K,V> {

    pub fn get(&self, key: &K) -> Result<V, Box<dyn Error>>
    
        where 
        
            K: std::cmp::PartialEq + std::cmp::Ord + Into<Vec<u8>> + Clone + TryFrom<Vec<u8>> + From<Vec<u8>>,
            
            V: Clone + TryFrom<Vec<u8>> + From<Vec<u8>>,
        
            <K as TryFrom<Vec<u8>>>::Error: std::error::Error,

            <V as TryFrom<Vec<u8>>>::Error: std::error::Error {
            
        match self.graves.iter().find(|&x| x == key) {

            Some(_) => Err("Not found!")?,

            None => {
                
                match self.cache.get(&key) {

                    Some(r) => Ok(r.clone()),

                    None => {

                        let key_bytes: Vec<u8> = key.clone().into();

                        let mut res: Option<V> = None;
                        
                        for table in &self.tables {

                            match res {

                                None => {

                                    match table.bloom_filter.search(&key_bytes) {
                                        
                                        true => {

                                            let table_path = format!(
                                                "{}/levels/{}/{}",
                                                &self.directory,
                                                table.level,
                                                table.name
                                            );

                                            let mut low = 0;

                                            let mut high = table.count as usize;

                                            while low != high {
                                                
                                                let mid_idx = (low + high) / 2;

                                                let mid_line: String = BufReader::new(
                                                        File::open(&table_path).unwrap()
                                                    )
                                                    .lines()
                                                    .nth(mid_idx + 2)
                                                    .unwrap()?;

                                                let spt_mid_line: Vec<&str> = mid_line.split(" ").collect();

                                                let mid_key: K = extract_key(&spt_mid_line)?;
                                                
                                                if key == &mid_key {

                                                    let val = extract_value(&spt_mid_line)?;

                                                    res = Some(val);
                                                    
                                                    break

                                                } else if key > &mid_key {
                                                    
                                                    low = mid_idx + 1

                                                } else {
                                                    
                                                    high = mid_idx - 1

                                                }

                                            }

                                        },

                                        false => ()

                                    }

                                },

                                _ => ()
                                
                            }

                        }

                        match res {
                            Some(r) => Ok(r),
                            None => Err("Not found!")?
                        }

                    }
                }
            }
        }
    }
}

fn extract_key<K>(arg: &[&str]) -> Result<K, Box<dyn Error>>

where K: From<Vec<u8>>

{
    
    if arg.len() == 2 {

        let k_bytes_hex = arg[0];
        
        let k_bytes = hex::decode(k_bytes_hex)?;
        
        let k = K::from(k_bytes);

        Ok(k)
    
    } else {
        
        Err("Not Supported!")?

    }

}

fn extract_value<V>(arg: &[&str]) -> Result<V, Box<dyn Error>>

where V: From<Vec<u8>>

{
    
    if arg.len() == 2 {

        let v_bytes_hex = arg[1];
        
        let v_bytes = hex::decode(v_bytes_hex)?;
        
        let v = V::from(v_bytes);

        Ok(v)
    
    } else {
        
        Err("Not Supported!")?

    }

}
