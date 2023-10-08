use fides::BloomFilter;
use crate::{Table, Store, KEY_INDEX_SIZE, TABLE_HEADER_SIZE};
use std::collections::{HashMap, BTreeMap, HashSet};
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{SeekFrom, Seek, Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

impl<K,V> Store<K,V> {
    
    pub fn compaction(&mut self) -> Result<(), Box<dyn Error>> {

        let mut unique_levels = HashSet::new();

        for table in &self.tables {
            unique_levels.insert(table.level);
        }

        for level in unique_levels {

            let tables: Vec<&Table> = self.tables.iter().filter(|x| x.level == level).collect();

            let level_size = tables.iter().fold(0, |acc, x| acc + x.file_size);

            if level_size > (10_u64.pow(level as u32) * 1_000_000) {

                let mut compaction_value_position = 0;

                let mut compaction_index: BTreeMap<
                    [u8;32], //key hash
                    (
                        u64, // key postion
                        u64, // key size
                        u64, // value position
                    )
                > =  BTreeMap::new();

                let mut compaction_key_locations: Vec<(
                    usize, // table index
                    u64, // key position
                    usize, // key size
                )> = Vec::new();

                let mut compaction_value_locations: Vec<(
                    usize, // table index,
                    u64, // value position
                    usize, // value size
                )> = Vec::new();

                let mut table_files: Vec<File> = Vec::new();

                for table in &tables {

                    let level_path = format!("{}/levels/{}", self.directory, table.level);

                    let table_path_str = format!("{}/{}.bin", &level_path, &table.name);
        
                    let table_path = Path::new(&table_path_str);

                    let table_file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(table_path)?;

                    table_files.push(table_file)

                }

                let mut level_value_positions: HashMap<[u8;32], u64> = HashMap::new();
                
                let mut keys_size: u64 = 0;

                for (i, table) in tables.iter().enumerate() {

                    match table_files.get_mut(i) {

                        Some(table_file) => {

                            table_file.seek(SeekFrom::Start(table.index_position))?;

                            while table_file.seek(SeekFrom::Current(0))? < table.keys_position {
                                
                                let mut key_hash = [0u8;32];
                                table_file.read_exact(&mut key_hash)?;
                                
                                if !compaction_index.contains_key(&key_hash) {

                                    let mut key_position_bytes = [0u8;8];
                                    table_file.read_exact(&mut key_position_bytes)?;
                                    let key_position = u64::from_le_bytes(key_position_bytes);

                                    let mut key_size_bytes = [0u8;8];
                                    table_file.read_exact(&mut key_size_bytes)?;

                                    let key_size = u64::from_le_bytes(key_size_bytes);

                                    let mut value_position_bytes = [0u8;8];
                                    table_file.read_exact(&mut value_position_bytes)?;

                                    let value_position = u64::from_le_bytes(value_position_bytes);

                                    let current_table_position = table_file.seek(SeekFrom::Current(0))?;

                                    table_file.seek(SeekFrom::Start(value_position))?;

                                    let mut value_hash = [0u8;32];
                                    table_file.read_exact(&mut value_hash)?;

                                    let current_value_position = match level_value_positions.get(&value_hash) {
                                        
                                        Some(r) => *r,
                                        
                                        None => {

                                            let mut value_size_buffer = [0u8;8];
                                            table_file.read_exact(&mut value_size_buffer)?;

                                            let value_size = u64::from_le_bytes(value_size_buffer);

                                            table_file.seek(SeekFrom::Start(current_table_position))?;

                                            let values_element_size = 32 + 8 + value_size;
                                            
                                            compaction_value_locations.push((
                                                i,
                                                value_position,
                                                values_element_size.try_into()?
                                            ));

                                            let current_value_position = compaction_value_position;

                                            compaction_value_position += values_element_size;

                                            level_value_positions.insert(value_hash, current_value_position);

                                            current_value_position

                                        }

                                    };

                                    compaction_key_locations.push((
                                        i,
                                        key_position,
                                        key_size as usize
                                    ));

                                    compaction_index.insert(
                                        key_hash,
                                        (keys_size, key_size, current_value_position)
                                    );

                                    keys_size += key_size;

                                }

                            }
                        }
                        None => todo!(),
                    }
                }

                let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

                let next_level_path = format!("{}/levels/{}", &self.directory, level + 1);

                fs::create_dir_all(&next_level_path)?;

                let compacted_path_str = format!("{}/{}.bin", &next_level_path, &current_time);

                let compacted_path = Path::new(&compacted_path_str);

                let mut compacted_file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(compacted_path)?;

                let mut bloom_filter = BloomFilter::new(compaction_index.len());

                for (key_hash, _) in &compaction_index {
                    bloom_filter.insert(key_hash);
                }

                let bloom_filter_bytes: Vec<u8> = (&bloom_filter).into();

                let key_count: u64 = compaction_index.len() as u64;

                let index_position = TABLE_HEADER_SIZE + bloom_filter_bytes.len() as u64;

                let keys_position = index_position + (key_count * KEY_INDEX_SIZE);

                let values_position = keys_position + keys_size;

                compacted_file.write_all(&[1u8])?;
                
                compacted_file.write_all(&key_count.to_le_bytes())?;
                compacted_file.write_all(&index_position.to_le_bytes())?;
                compacted_file.write_all(&keys_position.to_le_bytes())?;
                compacted_file.write_all(&bloom_filter_bytes)?;

                for (key_hash, (key_location, key_size, value_position)) in compaction_index {
                    
                    compacted_file.write_all(&key_hash)?;

                    compacted_file.write_all(&(keys_position + key_location).to_le_bytes())?;

                    compacted_file.write_all(&key_size.to_le_bytes())?;

                    compacted_file.write_all(&(values_position + value_position).to_le_bytes())?;

                }

                for (table_file_index, key_position, key_size) in compaction_key_locations {

                    table_files[table_file_index].seek(SeekFrom::Start(key_position))?;

                    let mut key_buffer = vec![0u8; key_size.try_into()?];

                    table_files[table_file_index].read_exact(&mut key_buffer)?;

                    compacted_file.write_all(&key_buffer)?;

                }

                for (table_file_index, value_position, value_size) in compaction_value_locations {
                    
                    table_files[table_file_index].seek(SeekFrom::Start(value_position))?;
                    
                    let mut value_buffer = vec![0u8; value_size];

                    table_files[table_file_index].read_exact(&mut value_buffer)?;

                    compacted_file.write_all(&value_buffer)?;

                }

                for table in tables {

                    let table_path = format!(
                        "{}/levels/{}/{}.bin",
                        &self.directory,
                        &table.level,
                        &table.name
                    );

                    fs::remove_file(table_path)?;

                }

                self.tables.retain(|x| x.level != level);

                let table = Table {
                    bloom_filter,
                    level: level + 1,
                    name: format!("{}", current_time),
                    key_count,
                    file_size: compacted_file.metadata()?.len(),
                    index_position,
                    keys_position,
                };

                self.tables.push(table);

                self.tables.sort_by(|a, b| b.name.cmp(&a.name));

            }

        }

        Ok(())

    }

}    
