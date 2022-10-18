use fides::BloomFilter;
use crate::{Store, Table};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::{BufRead, Read, SeekFrom, Seek};
use std::io::BufReader;
use std::path::Path;
use std::str;

impl<K: std::fmt::Debug,V: std::fmt::Debug> Store<K,V> {

    pub fn new(directory: &str) -> Result<Store<K,V>, Box<dyn Error>>
    
        where
        
            K: std::cmp::PartialEq + std::cmp::Ord + TryFrom<Vec<u8>> + Clone + From<Vec<u8>>,
            K: Clone + Into<Vec<u8>>, V: Clone + Into<Vec<u8>>,
            V: TryFrom<Vec<u8>> + Clone + From<Vec<u8>>,
            <K as TryFrom<Vec<u8>>>::Error: std::error::Error,
            <V as TryFrom<Vec<u8>>>::Error: std::error::Error {

        
        fs::create_dir_all(directory)?;

        let graves_location = format!("{}/graves.txt", &directory);

        let graves_path = Path::new(&graves_location);

        let mut graves: Vec<K> = Vec::new();

        if graves_path.is_file() {

            let graves_open = File::open(graves_path)?;

            let graves_buffer = BufReader::new(graves_open);

            for line in graves_buffer.lines() {
                
                let line = line?;

                let k_bytes = hex::decode(&line)?;

                    match K::try_from(k_bytes) {
                        
                        Ok(k) => graves.push(k),

                        _ => ()
                    
                    }
            
            }
            
        };

        let mut tables = Vec::new();

        let tables_location = format!("{}/levels", &directory);

        let tables_path = Path::new(&tables_location);

        if tables_path.is_dir() {

            for level in fs::read_dir(tables_path)? {

                let level = level?;
                
                let level_path = level.path();

                let level_name = level.file_name().into_string().unwrap();

                let level = u8::from_str_radix(&level_name, 10)?;

                if level_path.is_dir() {
                    
                    for table in fs::read_dir(level_path)? {

                        let table = table?;

                        let table_path = table.path();

                        let table_name = table.file_name().into_string().unwrap();

                        if table_path.is_file() {

                            let mut file = File::open(table_path)?;

                            let mut bloom_size_buffer = [0; 8];

                            file.read_exact(&mut bloom_size_buffer)?;

                            let bloom_size = usize::from_le_bytes(bloom_size_buffer);

                            let mut bloom_buffer = vec![0; bloom_size];

                            file.seek(SeekFrom::Start(8))?;

                            file.read_exact(&mut bloom_buffer)?;

                            let bloom_filter = BloomFilter::from(&bloom_buffer[..]);

                            let table = Table {
                                bloom_filter,
                                level,
                                name: table_name,
                                size: table.metadata()?.len(),
                            };

                            tables.push(table)

                        }

                    }
                }
            }
        }

        let mut cache: BTreeMap<K, V> = BTreeMap::new();

        let logs_location = format!("{}/logs.txt", &directory);

        let logs_path = Path::new(&logs_location);

        if logs_path.is_file() {

            let logs_open = File::open(logs_path)?;

            let logs_buffer = BufReader::new(logs_open);
            
            for line in logs_buffer.lines() {

                let line = line?;
                
                let split: Vec<&str> = line.split(' ').collect();
    
                match split[0] {
                    
                    "put" => {

                        let k_bytes = hex::decode(&split[1])?;

                        match K::try_from(k_bytes) {

                            Ok(k) => {

                                let v_bytes = hex::decode(&split[2])?;

                                match V::try_from(v_bytes) {

                                    Ok(v) => {

                                        graves.retain(|x| x != &k);
                        
                                        cache.insert(k, v);
                                    },

                                    _ => ()

                                }

                            },

                            _ => ()

                        }

                    },

                    "del" => {
                        
                        let k_bytes = hex::decode(&split[1])?;

                        match K::try_from(k_bytes) {
                            Ok(k) => graves.push(k),
                            _ => ()
                        }

                    },
                    
                    _ => ()
    
                }
            }
        }

        let mut store = Store {
            cache,
            directory: directory.to_string(),
            graves,
            tables
        };

        if logs_path.is_file() {

            if logs_path.metadata()?.len() > 1000000 {

                store.flush()?;
            
                fs::remove_file(logs_path)?;

                store.cache.clear();

                store.compaction()?;

            }

        }

        Ok(store)

    }
    
}
