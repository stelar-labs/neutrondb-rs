
use std::error::Error;

use std::fs;
use std::path::Path;

use std::time::{SystemTime, UNIX_EPOCH};

use stellar_notation::StellarObject;
use stellar_notation::StellarValue;

use crate::store;
use crate::bloom_filter;

pub fn perform(store: store::Store) -> Result<(), Box<dyn Error>> {

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    let store_path = format!("./neutrondb/{}", store.name);

    let l1_path = format!("{}/level_1", &store_path);

    if Path::new(&l1_path).is_dir() == false {
        fs::create_dir(&l1_path)?;
    }

    let cache_path = format!("{}/cache.stellar", &store_path);
        
    let cache_bytes = fs::read(&cache_path)?;

    let table_path = format!("{}/{}.stellar", &l1_path, &current_time);

    fs::write(&table_path, &cache_bytes)?;

    let empty_bloom_filter: Vec<u8> = vec![0; 32];
    // println!(" * empty_bloom_filter: {:?}", empty_bloom_filter);

    let new_bloom_filter = store.cache.iter().fold(empty_bloom_filter, |acc, x| bloom_filter::insert(acc, &x.0));
    // println!(" * new_bloom_filter: {:?}", new_bloom_filter);

    let new_bloom_filter_object = StellarObject(current_time.to_string(), StellarValue::ByteType(new_bloom_filter.clone()));

    let bloom_filters_path = format!("{}/bloom_filters.stellar", &store_path);

    if Path::new(&bloom_filters_path).is_file() {

        let current_bloom_filters = fs::read(&bloom_filters_path)?;
        let new_bloom_filters = stellar_notation::push(&current_bloom_filters, new_bloom_filter_object);
        fs::write(&bloom_filters_path, &new_bloom_filters)?;
    
    } else {

        let serialized_bloom_filters = stellar_notation::serialize(new_bloom_filter_object);
        let new_bloom_filters = stellar_notation::encode(&vec![serialized_bloom_filters]);
        fs::write(&bloom_filters_path, &new_bloom_filters)?;

    }

    let new_table_location_object = StellarObject(current_time.to_string(), StellarValue::IntegerType(1));

    let table_locations_path = format!("{}/table_locations.stellar", &store_path);

    if Path::new(&table_locations_path).is_file() {

        let current_table_locations = fs::read(&table_locations_path)?;
        let new_table_locations = stellar_notation::push(&current_table_locations, new_table_location_object);
        fs::write(&table_locations_path, &new_table_locations)?;
    
    } else {

        let serialized_table_location = stellar_notation::serialize(new_table_location_object);
        let new_table_locations = stellar_notation::encode(&vec![serialized_table_location]);
        fs::write(&table_locations_path, &new_table_locations)?;
        
    }

    Ok(())

}