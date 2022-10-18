use fides::BloomFilter;
use crate::{Table, Store, neutron};
use opis::Int;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

impl<K,V> Store<K,V> {
    
    pub fn compaction(&mut self) -> Result<(), Box<dyn Error>>
    
        where

            K: std::clone::Clone + std::cmp::PartialEq + std::cmp::Ord + Into<Vec<u8>> + From<Vec<u8>>,

            V: std::clone::Clone + Into<Vec<u8>> + From<Vec<u8>>
            
                {

                    for level in 1..=10 {

                        let mut tables: Vec<&Table> = self.tables
                            .iter()
                            .filter(|x| x.level == level)
                            .collect();

                        let level_size = tables
                            .iter()
                            .fold(0, |acc, x| acc + x.size);

                        if level_size > (10_u64.pow(level as u32) * 1000000) {

                            tables.sort_by_key(|x| x.name.to_string());

                            let level_path = format!("{}/tables/level_{}", &self.directory, &level);

                            let mut level_data : Vec<(K,V)> = Vec::new();

                            for table in tables {

                                let table_path = format!("{}/{}.neutron", &level_path, table.name);

                                let table_buffer = fs::read(&table_path).unwrap();

                                fs::remove_file(table_path).unwrap();

                                let table_objects = neutron::get_all::run(&table_buffer)?;

                                level_data = [level_data, table_objects].concat();

                            }

                            level_data.retain(|x| self.graves.contains(&x.0) == false);

                            level_data.reverse();

                            level_data.sort_by_key(|x| x.0.to_owned());

                            level_data.dedup_by_key(|x| x.0.to_owned());

                            let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

                            let next_level = level + 1;

                            let next_level_path = format!(
                                "{}/tables/level_{}",
                                &self.directory,
                                &next_level
                            );

                            fs::create_dir_all(&next_level_path)?;

                            let compact_path_str = format!(
                                "{}/{}.neutron",
                                &next_level_path,
                                &current_time
                            );

                            let compact_path = Path::new(&compact_path_str);

                            let empty_bloom_filter = BloomFilter::new(level_data.len());

                            let bloom_filter = level_data
                                .iter()
                                .fold(
                                    empty_bloom_filter,
                                    |mut acc, x| {

                                        let k = x.0.clone();

                                        let k_bytes: Vec<u8> = k.into();

                                        acc.insert(&k_bytes);
                                        
                                        acc
                                    
                                    }
                                );

                            let bloom_filter_bytes: Vec<u8> = Int::from(&bloom_filter.bits()[..]).into();

                            let table_map = level_data
                                .iter()
                                .fold(BTreeMap::new(), |mut acc, x| {
                                    acc.insert(x.0.clone(), x.1.clone());
                                    acc
                                });
                                
                            let table_buffer = neutron::create::run(
                                bloom_filter_bytes,
                                &table_map
                            );

                            fs::write(&compact_path, &table_buffer)?;

                            self.tables.retain(|x| x.level != level);

                            let table = Table {
                                bloom_filter,
                                level: next_level,
                                name: current_time.to_string(),
                                size: compact_path.metadata()?.len()
                            };

                            self.tables.push(table);

                        }

                    }

                    Ok(())
                }

}