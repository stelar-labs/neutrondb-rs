
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use stellar_notation::StellarObject;
use stellar_notation::StellarValue;

use crate::store::Store;
use crate::bloom_filter;

pub fn perform(mut store: Store) -> Result<(), Box<dyn Error>> {
    
    let store_path = format!("./neutrondb/{}", store.name);

    for level in 1..=4 {

        let dir_path = format!("{}/level_{}", &store_path, &level);

        if Path::new(&dir_path).is_dir() {

            let mut dir_files = Vec::new();

            for file in fs::read_dir(&dir_path)? {
                let file = file?;
                let file_path = file.path();
                if file_path.is_file() {
                    dir_files.push(file_path)
                }
            }

            if dir_files.len() == 5 {
                
                dir_files.sort();
                dir_files.reverse();

                let decoded_files_vec: Vec<Vec<Vec<u8>>> = dir_files.iter()
                    .map(|x| fs::read(x).unwrap())
                    .map(|x| stellar_notation::decode(&x))
                    .collect();

                let decoded_files = decoded_files_vec.concat();
                
                let mut objects: Vec<StellarObject> = decoded_files.iter()
                    .map(|x| stellar_notation::deserialize(x).unwrap())
                    .collect();

                let grave_list = store.grave.clone();

                for i in grave_list {

                    let objects_query = objects.iter()
                        .find(|x| x.0 == i);

                    match objects_query {

                        Some(_) => {

                            store.grave.retain(|x| x != &i);

                            objects.retain(|x| x.0 != i);

                        },

                        None => ()

                    }
                }

                if store.grave.len() > 0 {

                    let grave_objects: Vec<Vec<u8>> = store.grave.iter()
                        .map(|x| StellarObject(x.to_string(), StellarValue::IntegerType(0)))
                        .map(|x| stellar_notation::serialize(x))
                        .collect();

                    let grave_bytes = stellar_notation::encode(&grave_objects);

                    let grave_path = format!("{}/grave.stellar", &store_path);

                    fs::write(&grave_path, &grave_bytes)?;

                }

                objects.sort_by_key(|x| x.clone().0);
                objects.dedup_by_key(|x| x.clone().0);

                let objects_serialized: Vec<Vec<u8>> = objects.iter()
                    .map(|x| stellar_notation::serialize(x.to_owned()))
                    .collect();

                let objects_bytes = stellar_notation::encode(&objects_serialized);

                let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

                let next_level_path = format!("{}/level_{}", &store_path, &level + 1);

                fs::create_dir_all(&next_level_path)?;

                let compaction_path = format!("{}/{}.stellar", &next_level_path, &current_time);

                fs::write(&compaction_path, &objects_bytes)?;

                let empty_bloom_filter: Vec<u8> = vec![0; 32];

                let new_bloom_filter = objects.iter().fold(empty_bloom_filter, |acc, x| bloom_filter::insert(acc, &x.0));

                let new_bloom_filter_object = StellarObject(current_time.to_string(), StellarValue::ByteType(new_bloom_filter));

                let bloom_filters_path = format!("{}/bloom_filters.stellar", &store_path);

                let current_bloom_filters = fs::read(&bloom_filters_path)?;

                let new_bloom_filters = stellar_notation::push(&current_bloom_filters, new_bloom_filter_object);

                fs::write(&bloom_filters_path, &new_bloom_filters)?;

                let new_table_location_object = StellarObject(current_time.to_string(), StellarValue::IntegerType(&level + 1));

                let table_locations_path = format!("{}/table_locations.stellar", &store_path);

                let current_table_locations = fs::read(&table_locations_path)?;

                let new_table_locations = stellar_notation::push(&current_table_locations, new_table_location_object);

                fs::write(&table_locations_path, &new_table_locations)?;

                for file in dir_files {
                    fs::remove_file(file)?;
                    // remove blooms and locations 
                }
                
            }

        }

    }

    Ok(())
}