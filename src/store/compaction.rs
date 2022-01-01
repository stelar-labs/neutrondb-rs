
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

    for level in 1..=4 {

        let mut level_lists: Vec<&List> = store.lists
            .iter()
            .filter(|x| x.level == level)
            .collect();

        level_lists.sort_by_key(|x| x.name.to_string());

        if level_lists.len() == 5 {

            let level_path = format!("{}/level_{}", &store_path, &level);

            let mut list_paths = Vec::new();

            for list in level_lists {
                let list_path = format!("{}/{}.ndbl", &level_path, list.name);
                if Path::new(&list_path).is_file() {
                    list_paths.push(list_path);
                }
            }

            let lists_vec: Vec<Vec<(String, String)>> = list_paths
                .iter()
                .map(|x| fs::read(x).unwrap())
                .map(|x| list::deserialize::list(&x).unwrap())
                .collect();

            let mut merged_list = lists_vec.concat();

            merged_list.retain(|x| store.graves.contains(&x.0) == false);

            merged_list.reverse();

            merged_list.sort_by_key(|x| x.0.to_owned());

            merged_list.dedup_by_key(|x| x.0.to_owned());

            let bloom_filter: Vec<u8> = merged_list
                .iter()
                .fold(vec![0; 32], |acc, x| bloom_filter::insert(acc, &x.0));

            let merged_list_buffer: Vec<u8> = list::serialize::list(&merged_list);

            let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

            let next_level: u8 = level + 1;

            let next_level_path = format!("{}/level_{}", &store_path, &next_level);

            fs::create_dir_all(&next_level_path)?;

            let merged_list_path = format!("{}/{}.ndbl", &next_level_path, &current_time);

            fs::write(&merged_list_path, &merged_list_buffer)?;

            store.lists.retain(|x| x.level != level);

            let new_list = List{
                name: current_time.to_string(),
                level: next_level,
                bloom_filter: bloom_filter
            };

            store.lists.push(new_list);

            let updated_lists: Vec<(String, String)> = store.lists
                .iter()
                .map(|x| {

                    let table_value: String = encode::list(
                        &vec![
                            encode::u8(&x.level),
                            encode::bytes(&x.bloom_filter)
                        ]
                    );
        
                    (x.name.to_string(), table_value)

                })
                .collect();

            let updated_lists_buffer: Vec<u8> = list::serialize::list(&updated_lists);

            let lists_path = format!("{}/lists.ndbl", &store_path);

            fs::write(&lists_path, &updated_lists_buffer)?;

            for stale_path in list_paths {
                fs::remove_file(stale_path)?;
            }
            
        }

    }

    Ok(())
}