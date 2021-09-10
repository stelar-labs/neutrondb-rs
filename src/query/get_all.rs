
use std::error::Error;
use std::fs;

use crate::Store;

use stellar_notation::{ decoding };

pub fn run(store: &Store) -> Result<Option<Vec<(String, String)>>, Box<dyn Error>> {

    let store_path = format!("./neutrondb/{}", store.name);

    let mut group = vec![];

    for table in &store.tables {

        let table_path = format!("{}/level_{}/{}.stellar", &store_path, table.level, table.name);

        let table_buffer = fs::read(&table_path)?;

        let table_group = decoding::group(&table_buffer)?;

        group = [group, table_group].concat();

    }

    group = [group, store.cache.clone()].concat();

    if group.is_empty() {
        
        Ok(None)
    
    } else {

        group.reverse();

        group.sort_by_key(|x| x.0.to_string());

        group.dedup_by_key(|x| x.0.to_string());
        
        Ok(Some(group))

    }

}