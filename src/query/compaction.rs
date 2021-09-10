
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use stellar_notation::{ encoding, decoding };

use crate::Store;
use crate::Table;
use crate::query::bloom_filter;

pub fn run(store: &mut Store) -> Result<(), Box<dyn Error>> {
    
    let store_path = format!("./neutrondb/{}", store.name);

    for level in 1..=4 {

        let mut tables: Vec<&Table> = store.tables
            .iter()
            .filter(|x| x.level == level)
            .collect();

        tables.sort_by_key(|x| x.name.to_string());

        if tables.len() > 4 {

            let level_path = format!("{}/level_{}", &store_path, &level);

            let mut table_files = Vec::new();

            for table in tables {
                let table_path = format!("{}/{}.stellar", &level_path, table.name);
                if Path::new(&table_path).is_file() {
                    table_files.push(table_path)
                }
            }

            let table_groups: Vec<Vec<(String, String)>> = table_files
                .iter()
                .map(|x| fs::read(x).unwrap())
                .map(|x| decoding::group(&x).unwrap())
                .collect();

            let mut level_group = table_groups.concat();

            for grave in store.graves.clone() {

                let key_query = level_group.iter()
                    .find(|x| x.0 == grave);

                match key_query {
                    Some(_) => {
                        store.graves.retain(|x| x != &grave);
                        level_group.retain(|x| x.0 != grave);
                    },
                    None => ()
                }

            }

            let bloom_filter: Vec<u8> = level_group
                .iter()
                .fold(vec![0; 32], |acc, x| bloom_filter::insert(acc, &x.0));

            let level_buffer: Vec<u8> = encoding::group(level_group);

            let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

            let next_level = &level + 1;

            let next_level_path = format!("{}/level_{}", &store_path, &next_level);

            fs::create_dir_all(&next_level_path)?;

            let next_table_path = format!("{}/{}.stellar", &next_level_path, &current_time);

            fs::write(&next_table_path, &level_buffer)?;

            store.tables.retain(|x| x.level != level);

            let new_table = Table{
                name: current_time.to_string(),
                level: next_level,
                bloom_filter: bloom_filter
            };

            store.tables.push(new_table);

            let mut new_tables_group = vec![];

            for table in &store.tables {

                let table_value = encoding::group(vec![
                    ("level".to_string(), encoding::u128(&(table.level as u128))),
                    ("bloom_filter".to_string(), encoding::bytes(&table.bloom_filter))
                ]);

                new_tables_group.push((current_time.to_string(), encoding::bytes(&table_value)));

            }

            let new_tables_buffer = encoding::group(new_tables_group);

            let tables_path = format!("{}/tables.stellar", &store_path);

            fs::write(&tables_path, &new_tables_buffer)?;

            for table in table_files {
                fs::remove_file(table)?;
            }
            
        }

    }

    Ok(())
}