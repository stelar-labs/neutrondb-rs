
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use stellar_notation::{
    byte_encode,
    byte_decode,
    value_encode
};

use crate::Store;
use crate::query::bloom_filter;

pub fn run(mut store: Store) -> Result<(), Box<dyn Error>> {
    
    let store_path = format!("./neutrondb/{}", store.name);

    for level in 1..=4 {

        let level_path = format!("{}/level_{}", &store_path, &level);

        if Path::new(&level_path).is_dir() {

            let mut level_files = Vec::new();

            for file in fs::read_dir(&level_path)? {
                let file = file?;
                let file_path = file.path();
                if file_path.is_file() {
                    level_files.push(file_path)
                }
            }

            if level_files.len() == 5 {
                
                level_files.sort();
                level_files.reverse();

                let level_file_groups: Vec<Vec<(String, String)>> = level_files
                    .iter()
                    .map(|x| fs::read(x).unwrap())
                    .map(|x| byte_decode::group(&x).unwrap())
                    .collect();

                let mut level_group: Vec<(String, String)> = level_file_groups.concat();

                let initial_grave_size = store.grave.len();

                let grave_list = store.grave.clone();

                for i in grave_list {

                    let objects_query = level_group.iter()
                        .find(|x| x.0 == i);

                    match objects_query {

                        Some(_) => {

                            store.grave.retain(|x| x != &i);

                            level_group.retain(|x| x.0 != i);

                        },

                        None => ()

                    }
                }

                if store.grave.len() != initial_grave_size {

                    let grave_group: Vec<(String, String)> = store.grave
                        .iter()
                        .map(|x| (x.to_string(), value_encode::u128(&0)))
                        .collect();

                    let grave_buffer = byte_encode::group(grave_group);

                    let grave_path = format!("{}/grave.stellar", &store_path);

                    fs::write(&grave_path, &grave_buffer)?;

                }

                let sorted_buffer: Vec<u8> = byte_encode::group(level_group.clone());

                let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

                let next_level_path = format!("{}/level_{}", &store_path, &level + 1);

                fs::create_dir_all(&next_level_path)?;

                let sorted_path = format!("{}/{}.stellar", &next_level_path, &current_time);

                fs::write(&sorted_path, &sorted_buffer)?;

                // bloom filter 
                let bloom_filter: Vec<u8> = level_group
                    .iter()
                    .fold(vec![0; 32], |acc, x| bloom_filter::insert(acc, &x.0));

                let bloom = (current_time.to_string(), value_encode::bytes(&bloom_filter));

                let blooms_path = format!("{}/blooms.stellar", &store_path);

                let mut blooms_buffer = fs::read(&blooms_path)?;

                let mut blooms_group = byte_decode::group(&blooms_buffer)?;

                blooms_group.push(bloom);

                blooms_buffer = byte_encode::group(blooms_group);

                fs::write(&blooms_path, &blooms_buffer)?;


                // tables
                let table = (current_time.to_string(), value_encode::u128(&(&level + 1)));

                let tables_path = format!("{}/tables.stellar", &store_path);

                let mut tables_buffer = fs::read(&tables_path)?;

                let mut tables_group = byte_decode::group(&tables_buffer)?;

                tables_group.push(table);

                tables_buffer = byte_encode::group(tables_group);

                fs::write(&tables_path, &tables_buffer)?;

                // REMOVE COMPACTED FILES
                for file in level_files {

                    fs::remove_file(file)?;
                    // remove blooms and locations

                }
                
            }

        }

    }

    Ok(())
}