use crate::{neutron, Store};
use std::{fs, error::Error, path::Path};

impl Store {

    pub fn get_all(&self) -> Result<Vec<(String, String)>, Box<dyn Error>> {

        let mut res: Vec<(String, String)> = Vec::new();

        for table in &self.tables {

            let table_path_str = format!(
                "{}/tables/level_{}/{}.neutron",
                &self.directory,
                table.level,
                table.name
            );

            let table_path = Path::new(&table_path_str);

            let buffer = fs::read(table_path)?;

            res = [res, neutron::fetch(&buffer)?].concat();

        }

        let cache: Vec<(String, String)> = self.cache.iter().map(|(k,v)| (k.to_owned(),v.to_owned())).collect();

        res = [res, cache].concat();

        res.reverse();
        
        res.sort_by_key(|x| x.0.to_owned());
        
        res.dedup_by_key(|x| x.0.to_owned());
            
        Ok(res)

    }
}