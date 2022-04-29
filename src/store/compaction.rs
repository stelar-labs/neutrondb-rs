use crate::bloom::Bloom;
use crate::{Table, Store, neutron};
use opis::Int;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

impl Store {
    
    pub fn compaction(&mut self) -> Result<(), Box<dyn Error>> {

        for level in 1..=12 {

            let mut tables: Vec<&Table> = self.tables
                .iter()
                .filter(|x| x.level == level)
                .collect();

            let level_size = tables.iter().fold(0, |acc, x| acc + x.size);

            if level_size > (10_u64.pow(level as u32) * 1000000) {

                tables.sort_by_key(|x| x.name.to_string());

                let level_path = format!("{}/tables/level_{}/", &self.directory, &level);

                let mut level_data: Vec<(String, String)> = tables
                    .iter()
                    .fold(vec![], | acc, x | {

                        let table_path = format!("{}{}.neutron", &level_path, x.name);

                        let table_buffer = fs::read(&table_path).unwrap();

                        fs::remove_file(table_path).unwrap();

                        [acc, neutron::fetch(&table_buffer).unwrap()].concat()

                    });

                level_data.retain(|x| self.graves.contains(&x.0) == false);

                level_data.reverse();

                level_data.sort_by_key(|x| x.0.to_owned());

                level_data.dedup_by_key(|x| x.0.to_owned());

                let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

                let next_level = level + 1;

                let next_level_path = format!("{}/tables/level_{}", &self.directory, &next_level);

                fs::create_dir_all(&next_level_path).unwrap();

                let compact_path_str = format!("{}/{}.neutron", &next_level_path, &current_time);

                let compact_path = Path::new(&compact_path_str);

                let compact_bloom_filter = level_data
                    .iter()
                    .fold(
                        Bloom::new(self.cache.len()),
                        |acc, x|
                        { acc.insert(&x.0) }
                    );

                let compact_buffer = neutron::create(
                    Int { magnitude: compact_bloom_filter.bits.clone(), sign: false }.to_bytes(),
                    level_data.into_iter().collect()
                );

                fs::write(&compact_path, &compact_buffer)?;

                self.tables.retain(|x| x.level != level);

                let table = Table {
                    bloom: compact_bloom_filter,
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