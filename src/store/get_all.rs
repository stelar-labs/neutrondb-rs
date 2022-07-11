use crate::{neutron, Store};
use std::{fs, error::Error};

impl Store {

    pub fn get_all(&self) -> Result<Vec<(String, String)>, Box<dyn Error>> {

        let mut res: Vec<(String, String)> = Vec::new();

        for table in &self.tables {

            let table_path = format!(
                "{}/levels/{}/{}",
                &self.directory_location,
                table.level,
                table.name
            );

            let buffer = fs::read(table_path)?;

            res = [res, neutron::get_all(&buffer)?].concat();

        }

        let cache: Vec<(String, String)> = self.cache.iter().map(|(k,v)| (k.to_owned(),v.to_owned())).collect();

        res = [res, cache].concat();

        res.reverse();
        
        res.sort_by_key(|x| x.0.to_owned());
        
        res.dedup_by_key(|x| x.0.to_owned());
            
        Ok(res)

    }

}
