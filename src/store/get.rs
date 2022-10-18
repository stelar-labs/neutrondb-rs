use std::collections::BTreeMap;
use std::error::Error;

use crate::{neutron, Store};

impl<K,V> Store<K,V> {

    pub fn get(&self, key: &K) -> Result<V, Box<dyn Error>>
    
        where 
        
            K: std::cmp::PartialEq + std::cmp::Ord + Into<Vec<u8>> + Clone + TryFrom<Vec<u8>>,
            
            V: Clone + TryFrom<Vec<u8>>,
        
            <K as TryFrom<Vec<u8>>>::Error: std::error::Error,

            <V as TryFrom<Vec<u8>>>::Error: std::error::Error {
            
        match self.graves.iter().find(|&x| x == key) {

            Some(_) => Err("Not found!")?,

            None => {
                
                match self.cache.get(&key) {

                    Some(r) => Ok(r.clone()),

                    None => {

                        let key_bytes: Vec<u8> = key.clone().into();

                        let mut res: Result<V, Box<dyn Error>> = Err("Not found!")?;
                        
                        for table in &self.tables {

                            match table.bloom_filter.search(&key_bytes) {
                                
                                true => {

                                    let table_path = format!(
                                        "{}/levels/{}/{}",
                                        &self.directory,
                                        table.level,
                                        table.name
                                    );
                                    
                                    let key_values: BTreeMap<K,V> = neutron::get::run(
                                        &[key],
                                        &table_path
                                    )?;
                                
                                    if !key_values.is_empty() {

                                        let value = key_values.get(&key).unwrap().clone();
                                        
                                        res = Ok(value);

                                        break

                                    }

                                },

                                false => ()

                            }

                        }

                        res

                    }
                }
            }
        }
    }
}
