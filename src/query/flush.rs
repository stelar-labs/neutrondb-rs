
use std::error::Error;
use std::fs;
use std::path::Path;

use std::time::{SystemTime, UNIX_EPOCH};

use stellar_notation::{ encoding };

use crate::Store;
use crate::Table;
use crate::query::bloom_filter;

pub fn run(store: &mut Store) -> Result<(), Box<dyn Error>> {

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

    let new_table = Table{
        name: current_time.to_string(),
        level: 1,
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

    Ok(())

}