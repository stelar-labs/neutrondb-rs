use fides::BloomFilter;
use crate::{Store, Table, KeyObject, ValueObject, TABLE_HEADER_SIZE};
use std::collections::{HashMap, BTreeMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::{fs, mem};
use std::fs::OpenOptions;
use std::io::{self, Seek, SeekFrom};
use std::io::Read;
use std::path::Path;
use std::str;

impl<K,V> Store<K,V>
        
        where
            V: Into<Vec<u8>> + TryFrom<Vec<u8>>,
            <V as TryFrom<Vec<u8>>>::Error: std::error::Error + 'static,
    
    {

    pub fn new(directory: &str) -> Result<Store<K,V>, Box<dyn Error>> {
        
        if !Path::new(directory).is_dir() {
            fs::create_dir_all(directory)?;
        }

        let graves_location = format!("{}/graves.bin", &directory);

        let graves_path = Path::new(&graves_location);

        let mut graves: HashSet<[u8; 32]> = HashSet::new();

        if graves_path.is_file() {

            let mut graves_file = File::open(graves_path)?;

            let mut graves_buffer = [0u8; 32];

            loop {

                match graves_file.read_exact(&mut graves_buffer) {

                    Ok(_) => {graves.insert(graves_buffer);},

                    Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => break,

                    Err(_) => Err("Graves Read Error!")?

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

                let mut level_name = level.file_name().into_string().unwrap();

                level_name = level_name.trim_end_matches(".bin").to_string();

                let level = u8::from_str_radix(&level_name, 10)?;

                if level_path.is_dir() {
                    
                    for table in fs::read_dir(level_path)? {

                        let table = table?;

                        let table_path = table.path();

                        let table_file_metadata = fs::metadata(&table_path)?;

                        let file_size = table_file_metadata.len();

                        let name = table.file_name().into_string().unwrap();

                        if table_path.is_file() {

                            let mut table_file = File::open(&table_path)?;

                            table_file.seek(SeekFrom::Start(1))?;

                            let mut key_count_bytes = [0; 8];
                            table_file.read_exact(&mut key_count_bytes)?;
                            let key_count = u64::from_be_bytes(key_count_bytes);

                            let mut index_position_bytes = [0; 8];
                            table_file.read_exact(&mut index_position_bytes)?;
                            let index_position = u64::from_be_bytes(index_position_bytes);

                            let mut key_data_position_bytes = [0; 8];
                            table_file.read_exact(&mut key_data_position_bytes)?;
                            let key_data_position = u64::from_be_bytes(key_data_position_bytes);

                            let bloom_filter_size = index_position - TABLE_HEADER_SIZE;
                            let mut bloom_filter_bytes = vec![0; bloom_filter_size.try_into()?];
                            table_file.read_exact(&mut bloom_filter_bytes)?;
                            let bloom_filter = BloomFilter::try_from(&bloom_filter_bytes[..])?;
                            
                            let table = Table {
                                bloom_filter,
                                key_count,
                                level,
                                name,
                                file_size,
                                index_position,
                                key_data_position,
                            };

                            tables.push(table)

                        }
                    }

                    tables.sort_by(|a, b| b.name.cmp(&a.name));

                }
            }
        }

        let mut cache: BTreeMap<[u8;32], KeyObject> = BTreeMap::new();

        let mut values: HashMap<[u8;32], ValueObject<V>> = HashMap::new();

        let mut cache_size = 0;
        
        let logs_location = format!("{}/logs.bin", &directory);
        let logs_path = Path::new(&logs_location);
        let mut logs_file = OpenOptions::new().append(true).create(true).open(logs_path)?;
        let mut log_type = [0u8;1];
        loop {
            match logs_file.read_exact(&mut log_type) {
                Ok(_) => {
                    match log_type {
                        [1] => {
                            
                            let mut key_hash = [0u8;32];
                            logs_file.read_exact(&mut key_hash)?;

                            let mut value_hash = [0u8;32];
                            logs_file.read_exact(&mut value_hash)?;

                            let mut key_size_bytes = [0u8;8];
                            logs_file.read_exact(&mut key_size_bytes)?;
                            let key_log_position = logs_file.seek(SeekFrom::Current(0))?;
                            let key_size_u64 = u64::from_be_bytes(key_size_bytes);
                            let key_size = key_size_u64 as usize;
                            
                            // let mut key_buffer = vec![0u8; key_size];
                            // logs_file.read_exact(&mut key_buffer)?;

                            // let key = K::try_from(key_buffer)?;
                            
                            let cache_object = KeyObject {
                                value_hash,
                                key_size,
                                key_log_position,
                            };

                            cache.insert(key_hash, cache_object);

                        },
                        [2] => {
                            let mut value_hash = [0u8;32];
                            logs_file.read_exact(&mut value_hash)?;

                            let mut value_size_bytes = [0u8;8];
                            logs_file.read_exact(&mut value_size_bytes)?;
                            let value_log_position = logs_file.seek(SeekFrom::Current(0))?;
                            let value_size = u64::from_be_bytes(value_size_bytes) as usize;
                            let mut value_buffer = vec![0u8;value_size];
                            logs_file.read_exact(&mut value_buffer)?;

                            let value = V::try_from(value_buffer)?;

                            cache_size += mem::size_of_val(&value);

                            let value_object = ValueObject {
                                value,
                                value_size,
                                value_log_position,
                            };

                            values.insert(value_hash, value_object);

                        },
                        [3] => {
                            let mut grave_hash = [0u8;32];
                            logs_file.read_exact(&mut grave_hash)?;
                            graves.insert(grave_hash);
                        },
                        _ => break
                    }
                },

                Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => break,

                Err(_) => Err("Logs Read Error!")?

            }

        }


        let store = Store {
            cache,
            directory: directory.to_string(),
            graves,
            tables,
            logs_file,
            values,
            cache_size: cache_size.try_into()?,
            cache_limit: 1_000_000,
            phantom: std::marker::PhantomData,
        };

        Ok(store)

    }
    
}
