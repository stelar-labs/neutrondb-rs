
use astro_notation::encode;
use crate::Store;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

impl Store {

    pub fn put(&mut self, key: &str, value: &str) {
            
        self.cache.push((key.to_string(), value.to_string()));

        let logs_put: String = format!(
            "0x01 {} {}\n",
            encode::bytes(&key.to_string().into_bytes()),
            encode::bytes(&value.to_string().into_bytes())
        );

        let logs_path = format!("{}/logs", &self.directory);

        let mut logs_file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(Path::new(&logs_path))
            .unwrap();

        write!(logs_file, "{}", &logs_put).unwrap();

        if logs_file.metadata().unwrap().len() > 2097152 {

            self.flush();
        
            fs::remove_file(logs_path).unwrap();

            self.cache.clear();

            self.compaction();

        }

        match self.graves.iter().find(|&x| x == key) {
            Some(_) => self.graves.retain(|x| x != key),
            None => ()
        }

    }
}