
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

pub fn perform(store: Store) -> Result<(), Box<dyn Error>> {

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

    // ADD BLOOM FILTER
    let new_bloom_filter = store.cache.iter()
        .fold(vec![0; 32], |acc, x| bloom_filter::insert(acc, &x.0));

    let new_bloom_filter_object = StellarObject(current_time.to_string(), StellarValue::Bytes(new_bloom_filter.clone()));

    let bloom_filters_path = format!("{}/bloom_filters.stellar", &store_path);

    if Path::new(&bloom_filters_path).is_file() {

        let bloom_filters = fs::read(&bloom_filters_path)?;

        let mut deserialize_bloom_filters = deserialize_stellar_objects(&bloom_filters);

        deserialize_bloom_filters.push(new_bloom_filter_object);

        let new_bloom_filters = serialize_stellar_objects(&deserialize_bloom_filters);

        fs::write(&bloom_filters_path, &new_bloom_filters)?;
    
    } else {

        let new_bloom_filters = serialize_stellar_objects(vec![new_bloom_filter_object]);
        
        fs::write(&bloom_filters_path, &new_bloom_filters)?;

    }

    // ADD TABLE LOCATION
    let new_table_location_object = StellarObject(current_time.to_string(), StellarValue::UInt8(1));

    let table_locations_path = format!("{}/table_locations.stellar", &store_path);

    if Path::new(&table_locations_path).is_file() {

        let table_locations = fs::read(&table_locations_path)?;

        let mut deserialize_table_locations = deserialize_stellar_objects(&table_locations);

        deserialize_table_locations.push(new_table_location_object);

        let new_table_locations = serialize_stellar_objects(&deserialize_table_locations);

        fs::write(&table_locations_path, &new_table_locations)?;
    
    } else {

        let new_table_locations = serialize_stellar_objects(vec![new_table_location_object]);
        
        fs::write(&table_locations_path, &new_table_locations)?;
        
    }

    Ok(())

}