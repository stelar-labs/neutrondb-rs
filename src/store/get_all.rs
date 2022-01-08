use crate::list;
use crate::Store;
use std::error::Error;
use std::fs;

pub fn run(store: &Store) -> Result<Option<Vec<(String, String)>>, Box<dyn Error>> {

    let store_path = format!("./ndb/{}", store.name);

    let mut res: Vec<(String, String)> = Vec::new();

    for list in &store.lists {

        let list_path = format!("{}/{}.neutron", &store_path, list.name);

        let list_buffer = fs::read(&list_path)?;

        res = [res, list::deserialize::list(&list_buffer)?].concat();

    }

    if res.is_empty() {
        
        Ok(None)
    
    } else {

        res.reverse();

        res.sort_by_key(|x| x.0.to_owned());

        res.dedup_by_key(|x| x.0.to_owned());
        
        Ok(Some(res))

    }

}