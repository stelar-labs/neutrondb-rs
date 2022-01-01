
use std::error::Error;
use std::fs;
use std::path::Path;

use std::time::{SystemTime, UNIX_EPOCH};

use stellar_notation::{ encode };

use crate::list;
use crate::Store;
use crate::List;
use crate::store::bloom_filter;

pub fn run(store: &mut Store) -> Result<(), Box<dyn Error>> {

    let store_path = format!("./ndb/{}", store.name);

    let level_1_path = format!("{}/level_1", &store_path);

    if Path::new(&level_1_path).is_dir() == false {

        fs::create_dir(&level_1_path)?;
        
    }

    store.cache.reverse();

    store.cache.sort_by_key(|x| x.0.to_owned());

    store.cache.dedup_by_key(|x| x.0.to_owned());

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    let sorted_buffer: Vec<u8> = list::serialize::list(&store.cache);

    let sorted_path = format!("{}/{}.ndbs", &level_1_path, &current_time);

    fs::write(&sorted_path, &sorted_buffer)?;

    let bloom_filter = store.cache
        .iter()
        .fold(vec![0; 32], |acc, x| bloom_filter::insert(acc, &x.0));

    let list = List{
        name: current_time.to_string(),
        level: 1,
        bloom_filter: bloom_filter
    };

    store.lists.push(list);

    let lists: Vec<(String, String)> = store.lists
        .iter()
        .map(|x| {

            let list_value: String = encode::list(
                &vec![
                    encode::u8(&x.level),
                    encode::bytes(&x.bloom_filter)
                ]
            );

            (x.name.to_string(), list_value)

        })
        .collect();

    let lists_buffer = list::serialize::list(&lists);

    let lists_path = format!("{}/lists.ndbl", &store_path);

    fs::write(&lists_path, &lists_buffer)?;

    Ok(())

}