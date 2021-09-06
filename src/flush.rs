
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

    let nbf_kv = (current_time.to_string(), value_encode::bytes(&new_bloom_filter));

    let bfp = format!("{}/bloom_filters.stellar", &store_path);

    if Path::new(&bfp).is_file() {

        let bf_bytes = fs::read(&bfp)?;

        let mut bf = byte_decode::group(&bf_bytes);

        bf.push(nbf_kv);

        let nbf = byte_encode::group(bf);

        fs::write(&bfp, &nbf)?;
    
    } else {

        let nbf = byte_encode::object(&nbf_kv.0, &nbf_kv.1);
        
        fs::write(&bfp, &nbf)?;

    }

    // add table location 
    let ntl_kv = (current_time.to_string(), value_encode::u128(&0));

    let tlp = format!("{}/table_locations.stellar", &store_path);

    if Path::new(&tlp).is_file() {

        let tl_bytes = fs::read(&tlp)?;

        let mut tl = byte_decode::group(&tl_bytes);

        tl.push(ntl_kv);

        let ntl = byte_encode::group(tl);

        fs::write(&tlp, &ntl)?;
    
    } else {

        let ntl = byte_encode::object(&ntl_kv.0, &ntl_kv.1);
        
        fs::write(&tlp, &ntl)?;
        
    }

    Ok(())

}