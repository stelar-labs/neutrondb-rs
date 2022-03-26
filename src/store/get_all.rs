
use crate::{list, Store};
use std::fs;

impl Store {

    pub fn get_all(&self) -> Option<Vec<(String, String)>> {

        let mut res: Vec<(String, String)> = Vec::new();

        for table in &self.meta {

            let buf = fs::read(format!("{}/{}.neutron", &self.directory, table.name)).unwrap();

            res = [res, list::deserialize::list(&buf).unwrap()].concat();

        }

        res = [res, self.cache.clone()].concat();

        if res.is_empty() {
            None
        } 
        
        else {
            res.reverse();
            res.sort_by_key(|x| x.0.to_owned());
            res.dedup_by_key(|x| x.0.to_owned());
            
            Some(res)
        }

    }
}