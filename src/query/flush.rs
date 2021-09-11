
use std::error::Error;
use std::fs;
use std::path::Path;

use std::time::{SystemTime, UNIX_EPOCH};

use stellar_notation::{ encoding };

use crate::Store;
use crate::Table;
use crate::query::bloom_filter;

pub fn run(store: &mut Store) -> Result<(), Box<dyn Error>> {

    let store_path = format!("./neutrondb/{}", store.name);

    let level_1_path = format!("{}/level_1", &store_path);

    if Path::new(&level_1_path).is_dir() == false {
        fs::create_dir(&level_1_path)?;
    }

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    let table_path = format!("{}/{}.stellar", &level_1_path, &current_time);

    fs::write(&table_path, &store.cache_buffer)?;

    let bloom_filter = store.cache
        .iter()
        .fold(vec![0; 32], |acc, x| bloom_filter::insert(acc, &x.0));

    let table = Table{
        name: current_time.to_string(),
        level: 1,
        bloom_filter: bloom_filter
    };

    store.tables.push(table);

    let tables_group: Vec<(String, String)> = store.tables
        .iter()
        .map(|x| {

            let table_value = encoding::group(vec![
                ("level".to_string(), encoding::u128(&(x.level as u128))),
                ("bloom_filter".to_string(), encoding::bytes(&x.bloom_filter))
            ]);

            (x.name.to_string(), encoding::bytes(&table_value))

        })
        .collect();

    let tables_buffer = encoding::group(tables_group);

    let tables_path = format!("{}/tables.stellar", &store_path);

    fs::write(&tables_path, &tables_buffer)?;

    Ok(())

}