use astro_format::string;
use opis::Int;
use crate::bloom::Bloom;
use crate::{Store, Table};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::{BufRead, Read, SeekFrom, Seek};
use std::io::BufReader;
use std::path::Path;
use std::str;

impl Store {

    pub fn new(cache_size: &u64, name: &str) -> Result<Store, Box<dyn Error>> {

        let directory_location = format!("./neutrondb/{}", name);

        let graves_location = format!("{}/graves", &directory_location);

        let graves_path = Path::new(&graves_location);

        let mut graves = Vec::new();

        if graves_path.is_file() {

            let graves_open = File::open(graves_path)?;

            let graves_buffer = BufReader::new(graves_open);

            for line in graves_buffer.lines() {
                
                let line = line?;

                graves.push(line);
            
            }
            
        };

        let mut tables = Vec::new();

        let tables_location = format!("{}/levels", &directory_location);

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

                            let mut bloom_buffer_size_len = [0; 8];

                            file.read_exact(&mut bloom_buffer_size_len)?;

                            let bloom_buffer_size = usize::from_le_bytes(bloom_buffer_size_len);

                            let mut bloom_buffer = vec![0; bloom_buffer_size];

                            file.seek(SeekFrom::Start(8))?;

                            file.read_exact(&mut bloom_buffer)?;

                            let bloom = Bloom { bits: Int::from_bytes(&bloom_buffer).magnitude };

                            let table = Table {
                                bloom: bloom,
                                level: level,
                                name: table_name,
                                size: table.metadata()?.len(),
                            };

                            tables.push(table)

                        }

                    }
                }
            }
        }

        let mut cache = BTreeMap::new();

        let logs_location = format!("{}/logs", &directory_location);

        let logs_path = Path::new(&logs_location);

        if logs_path.is_file() {

            let logs_open = File::open(logs_path)?;

            let logs_buffer = BufReader::new(logs_open);
            
            for line in logs_buffer.lines() {

                let line = line?;
                
                let split: Vec<&str> = line.split(' ').collect();
    
                match split[0] {
                    
                    "put" => {

                        let k_bytes = string::decode::bytes(split[1])?;
                        
                        let k = String::from_utf8(k_bytes)?;

                        let v_bytes = string::decode::bytes(split[2])?;

                        let v = String::from_utf8(v_bytes)?;

                        graves.retain(|x| x != &k);
                        
                        cache.insert(k, v);
                        
                    },

                    "del" => {
                        
                        let k_bytes = string::decode::bytes(split[1])?;
                        
                        let k = String::from_utf8(k_bytes)?;

                        graves.push(k)

                    },
                    
                    _ => ()
    
                }
            }
        }

        let mut store = Store {
            cache: cache,
            cache_size: *cache_size,
            directory_location: directory_location,
            graves: graves,
            tables: tables
        };

        if logs_path.metadata()?.len() > store.cache_size {

            store.flush()?;
        
            fs::remove_file(logs_path)?;

            store.cache.clear();

            store.compaction()?;

        }

        Ok(store)

    }
    
}
