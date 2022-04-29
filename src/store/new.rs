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

    pub fn new(name: &str) -> Result<Store, Box<dyn Error>> {

        let directory = format!("./neutrondb/{}", name);

        let directory_path = Path::new(&directory);

        fs::create_dir_all(&directory_path)?;

        let graves_path_str = format!("{}/graves", &directory);

        let graves_path = Path::new(&graves_path_str);

        let mut graves = Vec::new();

        if graves_path.is_file() {

            for line in BufReader::new(File::open(graves_path)?).lines() {
                
                let line = line?;
                
                let decoded = string::decode::as_bytes(&line).unwrap();

                graves.push(str::from_utf8(&decoded)?.to_string());
            
            }
            
        };

        let mut tables = Vec::new();

        let tables_path_str = format!("{}/tables", &directory);

        let tables_path = Path::new(&tables_path_str);

        if tables_path.is_dir() {

            for level in fs::read_dir(tables_path)? {

                let level = level?;
                
                let level_path = level.path();

                let level_name: String = level.file_name().into_string().unwrap();

                let level_strs: Vec<&str> = level_name.split('_').collect();

                let level = u8::from_str_radix(level_strs[1], 10)?;

                if level_path.is_dir() {
                    
                    for table in fs::read_dir(level_path)? {

                        let table = table?;

                        let table_path = table.path();

                        let table_name = table.file_name().into_string().unwrap();

                        let table_strs: Vec<&str> = table_name.split('.').collect();

                        if table_path.is_file() {

                            let mut file = File::open(table_path)?;

                            let mut index_len_buffer = [0; 8];

                            file.read_exact(&mut index_len_buffer)?;

                            let index_len = u64::from_be_bytes(index_len_buffer);

                            let mut bloom_len_buffer = [0; 8];

                            file.read_exact(&mut bloom_len_buffer)?;

                            let bloom_len = usize::from_be_bytes(bloom_len_buffer);

                            let start = 16 + index_len;

                            file.seek(SeekFrom::Start(start))?;

                            let mut bf_buffer = vec![0; bloom_len];

                            file.read_exact(&mut bf_buffer)?;

                            let bloom = Bloom { bits: Int::from_bytes(&bf_buffer).magnitude };

                            let table = Table {
                                bloom: bloom,
                                level: level,
                                name: table_strs[0].to_string(),
                                size: table.metadata()?.len(),
                            };

                            tables.push(table)

                        }

                    }
                }
            }
        }

        let mut cache = BTreeMap::new();

        let logs_path_str = format!("{}/logs", &directory);

        let logs_path = Path::new(&logs_path_str);

        if logs_path.is_file() {
            
            for line in BufReader::new(File::open(logs_path).unwrap()).lines() {

                let line = line?;
                
                let split: Vec<&str> = line.split(' ').collect();
    
                match split[0] {
                    
                    "put" => {
                        
                        let key: String = str::from_utf8(&string::decode::as_bytes(split[1]).unwrap())?.to_string();

                        let value: String = str::from_utf8(&string::decode::as_bytes(split[2]).unwrap())?.to_string();
                        
                        graves.retain(|x| x != &key);
                        
                        cache.insert(key, value);
                        
                    },

                    "delete" => graves.push(str::from_utf8(&string::decode::as_bytes(split[1]).unwrap())?.to_string()),
                    
                    _ => ()
    
                }
            }
        }

        let res: Store = Store {
            directory: directory,
            cache: cache,
            graves: graves,
            tables: tables
        };

        Ok(res)

    }
}