use fides::BloomFilter;
use crate::{Store, Table};
use std::collections::HashMap;
use std::error::Error;
use std::{fs, vec};
use std::fs::{OpenOptions, File};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{Write, Seek, SeekFrom, Read};

impl<'a,K,V> Store<K,V> {

    pub fn flush(&mut self) -> Result<(), Box<dyn Error>> {

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let level_path = format!("{}/levels/1", self.directory);
        
        fs::create_dir_all(&level_path)?;

        let table_path_str = format!("{}/{}.bin", &level_path, &current_time);
        
        let table_path = Path::new(&table_path_str);

        let mut table_file = OpenOptions::new().append(true).create(true).open(table_path)?;
        
        let mut bloom_filter = BloomFilter::new(self.cache.len());

        let mut key_position: u64 = 0;

        let mut value_position: u64 = 0;

        let mut index: Vec<([u8;32], u64, u64)> = Vec::new();

        let mut value_positions: HashMap<[u8;32],u64> = HashMap::new();

        let mut value_data_offset = 0;

        let mut value_data_order = Vec::new();

        for (key_hash, cache_object) in &self.cache {

            value_data_offset += cache_object.key_size as u64;

            bloom_filter.insert(key_hash);

            value_data_order.push(cache_object.value_hash);

            let value_position = match value_positions.get(&cache_object.value_hash) {
                Some(r) => *r,
                None => {
                    match self.values.get(&cache_object.value_hash) {
                        Some(value_object) => {
                            let current_val_pos = value_position;
                            value_positions.insert(cache_object.value_hash, value_position);
                            value_position += value_object.value_size as u64;
                            current_val_pos
                        },
                        None => value_position,
                    }
                },
            };

            index.push((*key_hash, key_position, value_position));

            key_position += cache_object.key_size as u64;

        }

        let bloom_filter_bytes: Vec<u8> = (&bloom_filter).into();

        let key_count = self.cache.len() as u64;

        let index_position = 1 + 24 + bloom_filter_bytes.len() as u64;

        let key_data_position = index_position + (key_count * (32 + 32 + 8 + 8));

        value_data_offset += key_data_position;
        
        for (_, key_pos, val_pos) in &mut index {
            *key_pos += key_data_position;
            *val_pos += value_data_offset;
        }

        table_file.write_all(&[1u8])?;

        table_file.write_all(&key_count.to_be_bytes())?;

        table_file.write_all(&index_position.to_be_bytes())?;

        table_file.write_all(&key_data_position.to_be_bytes())?;

        table_file.write_all(&bloom_filter_bytes)?;

        for (_, cache_object) in &self.cache {
            let mut key_bytes = vec![0u8; cache_object.key_size];
            self.logs_file.seek(SeekFrom::Start(cache_object.key_log_position))?;
            self.logs_file.read_exact(&mut key_bytes)?;
            table_file.write_all(&key_bytes)?;
        }

        for value_hash in value_data_order {
            match self.values.get(&value_hash) {
                Some(value_object) => {
                    let mut value_bytes = vec![0u8; value_object.value_size];
                    self.logs_file.seek(SeekFrom::Start(value_object.value_log_position))?;
                    self.logs_file.read_exact(&mut value_bytes)?;
                    table_file.write_all(&value_bytes)?;
                },
                None => (),
            }
        }

        let table = Table {
            bloom_filter,
            level: 1_u8,
            name: format!("{}", current_time),
            key_count,
            file_size: table_file.metadata()?.len(),
            index_position,
            key_data_position
        };

        self.tables.push(table);

        self.tables.sort_by(|a, b| b.name.cmp(&a.name));

        let graves_path_str = format!("{}/graves.bin", &self.directory);

        let graves_path = Path::new(&graves_path_str);

        if graves_path.is_file() {

            let mut graves_file = File::open(graves_path)?;

            graves_file.set_len(0)?;

            for grave in &self.graves {
                graves_file.write_all(grave)?;
            }


        };

        Ok(())

    }

}
