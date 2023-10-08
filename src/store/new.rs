use fides::BloomFilter;
use crate::types::into_bytes::IntoBytes;
use crate::types::try_from_bytes::TryFromBytes;
use crate::{Store, Table, KeyObject, ValueObject, TABLE_HEADER_SIZE};
use std::collections::{HashMap, BTreeMap, HashSet};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::{fs, mem};
use std::fs::OpenOptions;
use std::io::{self, Seek, SeekFrom};
use std::io::Read;
use std::path::Path;
use std::str;


impl<K,V> Store<K,V>
        
        where
            V: IntoBytes + TryFromBytes + std::fmt::Debug
    
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

                let level_name = level.file_name().into_string().unwrap();

                let level = u8::from_str_radix(&level_name, 10)?;

                if level_path.is_dir() {
                    
                    for table in fs::read_dir(level_path)? {

                        let table = table?;

                        let table_path = table.path();

                        let table_file_metadata = fs::metadata(&table_path)?;

                        let file_size = table_file_metadata.len();

                        let name = table_path
                            .file_stem()
                            .and_then(OsStr::to_str)
                            .ok_or_else(|| Box::<dyn Error>::from("Invalid filename"))?.to_string();

                        if table_path.is_file() {

                            let mut table_file = File::open(&table_path)?;

                            table_file.seek(SeekFrom::Start(1))?;

                            let mut key_count_bytes = [0; 8];
                            table_file.read_exact(&mut key_count_bytes)?;
                            let key_count = u64::from_le_bytes(key_count_bytes);

                            let mut index_position_bytes = [0; 8];
                            table_file.read_exact(&mut index_position_bytes)?;
                            let index_position = u64::from_le_bytes(index_position_bytes);

                            let mut key_data_position_bytes = [0; 8];
                            table_file.read_exact(&mut key_data_position_bytes)?;
                            let keys_position = u64::from_le_bytes(key_data_position_bytes);

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
                                keys_position,
                            };

                            tables.push(table)

                        }
                    }

                    tables.sort_by(|a, b| b.name.cmp(&a.name));

                }
            }
        }

        let mut keys: BTreeMap<[u8;32], KeyObject> = BTreeMap::new();

        let mut values: HashMap<[u8;32], ValueObject<V>> = HashMap::new();

        let mut cache_size = 0;
        
        let logs_location = format!("{}/logs.bin", &directory);

        let logs_path = Path::new(&logs_location);

        let mut logs_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(logs_path)?;
        
        let mut log_type_byte = [0u8;1];

        if logs_file.metadata()?.len() != 0 {
            
            loop {

                match logs_file.read_exact(&mut log_type_byte) {

                    Ok(_) => {

                        match log_type_byte {

                            [1] => {
                                
                                let mut key_hash = [0u8;32];
                                logs_file.read_exact(&mut key_hash)?;

                                let mut value_hash = [0u8;32];
                                logs_file.read_exact(&mut value_hash)?;

                                let mut key_size_bytes = [0u8;8];
                                logs_file.read_exact(&mut key_size_bytes)?;

                                let key_size_u64 = u64::from_le_bytes(key_size_bytes);
                                let key_size = key_size_u64 as usize;

                                let key_log_position = logs_file.seek(SeekFrom::Current(0))?;
                                
                                let key_object = KeyObject {
                                    value_hash,
                                    key_size,
                                    key_log_position,
                                };


                                // move the pointer past key bytes
                                logs_file.seek(SeekFrom::Current(key_size_u64 as i64))?;

                                keys.insert(key_hash, key_object);

                            },

                            [2] => {

                                let value_log_position = logs_file.seek(SeekFrom::Current(0))?;

                                let mut value_hash = [0u8;32];
                                logs_file.read_exact(&mut value_hash)?;

                                let mut value_size_buffer = [0u8;8];
                                logs_file.read_exact(&mut value_size_buffer)?;
                                let value_size = u64::from_le_bytes(value_size_buffer) as usize;
                                
                                let mut value_buffer = vec![0u8;value_size];
                                logs_file.read_exact(&mut value_buffer)?;

                                let value = V::try_from_bytes(value_buffer)?;

                                cache_size += mem::size_of_val(&value);

                                let value_object = ValueObject {
                                    value,
                                    value_size: 32 + 8 +value_size,
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

                    // Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => break,

                    // Err(_) => Err("Logs Read Error!")?

                    Err(_) => break

                }

            }
        }

        let store = Store {
            keys,
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
