
use crate::{bloom, list, Store};
use std::path::Path;

impl Store {

    pub fn get(&self, key: &str) -> Option<String> {
            
        match self.graves.iter().find(|&x| x == key) {
            Some(_) => None,
            None => {
                
                match self.cache.iter().find(|x| x.0 == key) {
                    Some(r) => Some(r.1.to_owned()),
                    None => {

                        let mut search_res: Option<String> = None;
                        
                        for table in self.meta.clone() {

                            if bloom::lookup(&table.bloom_filter, &key) {
                                
                                let list_path = format!("{}/tables/level_{}/{}.neutron", &self.directory, table.level, table.name);
                                
                                match list::search::key(Path::new(&list_path), key).unwrap() {
                                    Some(r) => {
                                        search_res = Some(r);
                                        break
                                    },
                                    None => ()
                                }

                            }

                        }

                        search_res

                    }
                }

            }

        }

    }
}