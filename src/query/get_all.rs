
use std::error::Error;
use std::fs;

use crate::Store;

use stellar_notation::{
    byte_decode
};

pub fn run(store: &Store) -> Result<Option<Vec<(String, String)>>, Box<dyn Error>> {

    let store_path = format!("./neutrondb/{}", store.name);

    let mut group = store.cache.clone();
    
    group.reverse();

    for table in &store.tables {

        let sorted_path = format!("{}/level_{}/{}.stellar", &store_path, table.1, table.0);

        let sorted_buffer = fs::read(&sorted_path)?;

        let sorted_group = byte_decode::group(&sorted_buffer)?;

        group = [group, sorted_group].concat();

    }

    if group.is_empty() {
        
        Ok(None)
    
    } else {

        group.sort_by_key(|x| x.0.to_string());

        group.dedup_by_key(|x| x.0.to_string());
        
        Ok(Some(group))

    }

}