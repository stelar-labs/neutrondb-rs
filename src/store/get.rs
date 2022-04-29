use crate::{neutron, Store};
use std::path::Path;

impl Store {

    pub fn get(&self, key: &str) -> Option<String> {
            
        match self.graves.iter().find(|&x| x == key) {

            Some(_) => None,

            None => {
                
                match self.cache.get(key) {

                    Some(r) => Some(r.to_string()),

                    None => {

                        let mut res: Option<String> = None;
                        
                        for table in &self.tables {

                            println!(" * search: {:?}", table.bloom.search(key));

                            match table.bloom.search(key) {
                                
                                true => {

                                    let table_path_str = format!(
                                        "{}/tables/level_{}/{}.neutron",
                                        &self.directory,
                                        table.level,
                                        table.name
                                    );
                                    
                                    let table_path = Path::new(&table_path_str);
                                
                                    match neutron::search(key, table_path) {
                                        
                                        Ok(r) => {
                                            res = Some(r);
                                            break
                                        },
                                        
                                        Err(_) => ()
                                    
                                    }

                                },

                                false => ()

                            }

                        }

                        res
                    }
                }
            }
        }
    }
}
