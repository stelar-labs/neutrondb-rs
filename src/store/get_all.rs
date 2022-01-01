
use std::error::Error;
use std::fs;

use crate::list;
use crate::Store;

pub fn run(store: &Store) -> Result<Option<Vec<(String, String)>>, Box<dyn Error>> {

    let store_path = format!("./ndb/{}", store.name);

    let mut all_list: Vec<(String, String)> = vec![];

    for list in &store.lists {

        let list_path = format!("{}/level_{}/{}.ndbl", &store_path, list.level, list.name);

        let list_buffer = fs::read(&list_path)?;

        all_list = [all_list, list::deserialize::list(&list_buffer)?].concat();

    }

    all_list = [all_list, store.cache.clone()].concat();

    if all_list.is_empty() {
        
        Ok(None)
    
    } else {

        all_list.reverse();

        all_list.sort_by_key(|x| x.0.to_string());

        all_list.dedup_by_key(|x| x.0.to_string());
        
        Ok(Some(all_list))

    }

}