use std::fs::File;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use crate::Store;

impl<'a, K,V> Store<K,V> {

    pub fn get(&self, key: &'a K) -> Result<V, Box<dyn Error>>
        
        where
            V: Clone + TryFrom<Vec<u8>, Error = Box<dyn Error>>,
            &'a K: Into<Vec<u8>>
    
    {

        let key_bytes: Vec<u8> = key.into();

        let key_hash = fides::hash::blake_3(&key_bytes);
            
        match self.graves.iter().find(|&x| x == &key_hash) {

            Some(_) => Err("Not found!")?,

            None => {
                
                match self.cache.get(&key_hash) {

                    Some(r) => {

                        match self.values.get(&r.value_hash) {
                            Some(value_object) => Ok(value_object.value.clone()),
                            None => Err("Not found!")?,
                        }
                        
                    },

                    None => {

                        let mut res: Option<V> = None;
                        
                        for table in &self.tables {

                            match res {

                                None => {

                                    match table.bloom_filter.search(&key_bytes) {
                                        
                                        true => {

                                            let table_path = format!("{}/levels/{}/{}.bin", &self.directory, table.level, table.name);

                                            let mut table_file = File::open(&table_path)?;

                                            let mut low = 0;

                                            let mut high = table.key_count;

                                            while low != high {
                                                
                                                let mid_pos = table.index_position + (((high - low) / 2) * 80);

                                                let mut table_key_hash = [0u8;32];
                                                table_file.seek(SeekFrom::Start(mid_pos))?;
                                                table_file.read_exact(&mut table_key_hash)?;
                                                
                                                if key_hash == table_key_hash {

                                                    table_file.seek(SeekFrom::Current(48))?;

                                                    let mut value_position_bytes = [0u8;8];
                                                    table_file.read_exact(&mut value_position_bytes)?;

                                                    let value_position = u64::from_be_bytes(value_position_bytes);

                                                    table_file.seek(SeekFrom::Start(value_position))?;

                                                    let mut value_size_buffer = [0u8;8];
                                                    table_file.read_exact(&mut value_size_buffer)?;
                                                    let value_size = u64::from_be_bytes(value_size_buffer) as usize;

                                                    let mut value_buffer = vec![0u8; value_size];
                                                    table_file.read_exact(&mut value_buffer)?;
                                                    
                                                    let value = V::try_from(value_buffer)?;
                                            
                                                    res = Some(value);
                                                    
                                                    break

                                                } else if key_hash > table_key_hash {
                                                    
                                                    low = mid_pos + 1

                                                } else {
                                                    
                                                    high = mid_pos - 1
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
