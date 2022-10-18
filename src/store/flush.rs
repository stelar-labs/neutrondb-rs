use fides::BloomFilter;
use crate::{neutron, Store, Table};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use opis::Int;

impl<K,V> Store<K,V> {

    pub fn flush(&mut self) -> Result<(), Box<dyn Error>>
    
        where
        
            K: Clone + Into<Vec<u8>>,
            
            V: Clone + Into<Vec<u8>>
        
                {

                    let level_path = format!("{}/levels/1",self.directory);
                    
                    fs::create_dir_all(&level_path)?;

                    let current_time = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis();

                    let table_path = format!("{}/{}.neutron", &level_path, &current_time);
                    
                    let table_location = Path::new(&table_path);

                    let empty_bloom_filter = BloomFilter::new(self.cache.len());

                    let bloom_filter = self.cache
                        .iter()
                        .fold(
                            empty_bloom_filter,
                            |mut acc, x| {
                                
                                let k_bytes: Vec<u8> = x.0.clone().into();

                                acc.insert(&k_bytes);
                                
                                acc
                            
                            }
                        );

                    let table_buffer = neutron::create::run(
                        Int::from(&bloom_filter.bits()[..]).into(),
                        &self.cache
                    );

                    fs::write(table_location, &table_buffer)?;

                    let table = Table {
                        bloom_filter,
                        level: 1_u8,
                        name: format!("{}", current_time),
                        size: table_location.metadata()?.len()
                    };

                    self.tables.push(table);

                    let graves_path = format!("{}/graves.txt", &self.directory);

                    let graves_location = Path::new(&graves_path);

                    if graves_location.is_file() {

                        fs::remove_file(graves_location)?;

                        let graves_input = self.graves
                            .iter()
                            .fold(
                                String::new(),
                                |mut acc, x| {

                                    let g_bytes: Vec<u8> = x.clone().into();

                                    acc = format!("{}{}\n", acc, hex::encode(g_bytes));

                                    acc
                                    
                                }
                            );

                        fs::write(graves_location, graves_input)?;

                    };

                    Ok(())

                }

}
