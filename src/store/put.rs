use astro_format::string;
use crate::Store;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

impl Store {

    pub fn put(&mut self, key: &str, value: &str) -> Result<(), Box<dyn Error>> {

        self.cache.insert(key.to_string(), value.to_string());

        let k_bytes = key.as_bytes();

        let v_bytes = value.as_bytes();

        let logs_put: String = format!(
            "put {} {}\n",
            string::encode::bytes(k_bytes),
            string::encode::bytes(v_bytes)
        );

        let logs_path_str = format!("{}/logs", &self.directory_location);

        let logs_path = Path::new(&logs_path_str);

        let mut logs_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(logs_path)?;

        write!(logs_file, "{}", &logs_put)?;

        if logs_file.metadata()?.len() > self.cache_size {

            self.flush()?;
        
            fs::remove_file(logs_path)?;

            self.cache.clear();

            self.compaction()?;

        }

        self.graves.retain(|x| *x != key);

        Ok(())

    }
}