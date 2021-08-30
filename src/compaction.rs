
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use stellar_notation::{
    StellarObject, StellarValue,
    serialize_stellar_objects, deserialize_stellar_objects
};

use crate::Store;
use crate::bloom_filter;

pub fn perform(mut store: Store) -> Result<(), Box<dyn Error>> {
    
    let store_path = format!("./neutrondb/{}", store.name);

    for level in 1..=4 {

        let dir_path = format!("{}/level_{}", &store_path, &level);

        if Path::new(&dir_path).is_dir() {

            let mut level_files = Vec::new();

            for file in fs::read_dir(&dir_path)? {
                let file = file?;
                let file_path = file.path();
                if file_path.is_file() {
                    level_files.push(file_path)
                }
            }

            if level_files.len() == 5 {
                
                level_files.sort();
                level_files.reverse();

                let level_file_objects_vec: Vec<Vec<StellarObject>> = level_files.iter()
                    .map(|x| fs::read(x).unwrap())
                    .map(|x| deserialize_stellar_objects(&x))
                    .collect();

                let mut level_objects: Vec<StellarObject> = level_file_objects_vec.concat();

                let initial_grave_size = store.grave.len();

                let grave_list = store.grave.clone();

                for i in grave_list {

                    let objects_query = level_objects.iter()
                        .find(|x| x.0 == i);

                    match objects_query {

                        Some(_) => {

                            store.grave.retain(|x| x != &i);

                            level_objects.retain(|x| x.0 != i);

                        },

                        None => ()

                    }
                }

                if store.grave.len() != initial_grave_size {

                    let grave_objects: Vec<StellarObject> = store.grave.iter()
                    .map(|x| StellarObject(x.to_string(), StellarValue::UInt8(0)))
                    .collect();

                    let grave_serialized = serialize_stellar_objects(grave_objects);

                    let grave_path = format!("{}/grave.stellar", &store_path);

                    fs::write(&grave_path, &grave_serialized)?;

                }

                let objects_serialized: Vec<u8> = serialize_stellar_objects(level_objects.clone());

                let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

                let next_level_path = format!("{}/level_{}", &store_path, &level + 1);

                fs::create_dir_all(&next_level_path)?;

                let compaction_path = format!("{}/{}.stellar", &next_level_path, &current_time);

                fs::write(&compaction_path, &objects_serialized)?;


                // ADD BLOOM FILTER
                let new_bloom_filter: Vec<u8> = level_objects.iter()
                    .fold(vec![0; 32], |acc, x| bloom_filter::insert(acc, &x.0));

                let new_bloom_filter_object = StellarObject(current_time.to_string(), StellarValue::Bytes(new_bloom_filter));

                let bloom_filters_path = format!("{}/bloom_filters.stellar", &store_path);

                let bloom_filters = fs::read(&bloom_filters_path)?;

                let mut deserialize_bloom_filters = deserialize_stellar_objects(&bloom_filters);

                deserialize_bloom_filters.push(new_bloom_filter_object);

                let new_bloom_filters = serialize_stellar_objects(deserialize_bloom_filters);

                fs::write(&bloom_filters_path, &new_bloom_filters)?;


                // ADD TABLE LOCATION
                let new_table_location_object = StellarObject(current_time.to_string(), StellarValue::UInt8(&level + 1));

                let table_locations_path = format!("{}/table_locations.stellar", &store_path);

                let table_locations = fs::read(&table_locations_path)?;

                let mut deserialize_table_locations = deserialize_stellar_objects(&table_locations);

                deserialize_table_locations.push(new_table_location_object);

                let new_table_locations = serialize_stellar_objects(deserialize_table_locations);

                fs::write(&table_locations_path, &new_table_locations)?;

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