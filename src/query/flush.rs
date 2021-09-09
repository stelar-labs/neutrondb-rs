
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

pub fn run(store: Store) -> Result<(), Box<dyn Error>> {

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    let store_path = format!("./neutrondb/{}", store.name);

    let l1_path = format!("{}/level_1", &store_path);

    if Path::new(&l1_path).is_dir() == false {
        fs::create_dir(&l1_path)?;

    }

    let table_path = format!("{}/{}.stellar", &l1_path, &current_time);

    fs::write(&table_path, &store.cache_buffer)?;

    let bloom_filter = store.cache
        .iter()
        .fold(vec![0; 32], |acc, x| bloom_filter::insert(acc, &x.0));

    let bloom = (current_time.to_string(), value_encode::bytes(&bloom_filter));

    let blooms_path = format!("{}/blooms.stellar", &store_path);

    if Path::new(&blooms_path).is_file() {

        let mut blooms_buffer = fs::read(&blooms_path)?;

        let mut blooms_group = byte_decode::group(&blooms_buffer)?;

        blooms_group.push(bloom);

        blooms_buffer = byte_encode::group(blooms_group);

        fs::write(&blooms_path, &blooms_buffer)?;
    
    } else {
        let blooms_buffer = byte_encode::object(&bloom.0, &bloom.1);
        fs::write(&blooms_path, &blooms_buffer)?;

    }

    // add table location 
    let table = (current_time.to_string(), value_encode::u128(&0));

    let tables_path = format!("{}/tables.stellar", &store_path);

    if Path::new(&tables_path).is_file() {

        let mut table_buffer = fs::read(&tables_path)?;

        let mut table_group = byte_decode::group(&table_buffer)?;

        table_group.push(table);

        let table_buffer = byte_encode::group(table_group);

        fs::write(&tables_path, &table_buffer)?;
    
    } else {

        let table_buffer = byte_encode::object(&table.0, &table.1);
        
        fs::write(&tables_path, &table_buffer)?;
        
    }

    Ok(())

}