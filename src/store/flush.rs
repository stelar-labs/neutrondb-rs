use astro_format::string;
use crate::bloom::Bloom;
use crate::{neutron, Store, Table};
use opis::Int;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

impl Store {

    pub fn flush(&mut self) -> Result<(), Box<dyn Error>> {

        let level_1_path_str = format!(
            "{}/tables/level_1",
            self.directory
        );

        let level_1_path = Path::new(&level_1_path_str);
        
        fs::create_dir_all(&level_1_path)?;

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let table_path_str = format!(
            "{}/{}.neutron",
            &level_1_path_str,
            &current_time
        );
        
        let table_path = Path::new(&table_path_str);

        let bloom = self.cache
            .iter()
            .fold(
                Bloom::new(self.cache.len()),
                |acc, x|
                { acc.insert(&x.0) }
            );

        let table_buffer = neutron::create(
            Int { magnitude: bloom.bits.clone(), sign: false }.to_bytes(),
            self.cache.clone()
        );

        fs::write(table_path, &table_buffer)?;

        let table: Table = Table {
            bloom: bloom,
            level: 1_u8,
            name: format!("{}", current_time),
            size: table_path.metadata()?.len()
        };

        self.tables.push(table);

        let graves_path_str = format!(
            "{}/graves",
            &self.directory
        );

        let graves_path = Path::new(&graves_path_str);

        if graves_path.is_file() {

            fs::remove_file(graves_path)?;

            let graves_input = self.graves
                .iter()
                .fold(
                    String::new(),
                    |acc, x|
                    format!("{}{}\n", acc, string::encode::bytes(&x.clone().into_bytes()))
                );

            fs::write(graves_path, graves_input)?;

        };

        Ok(())

    }

}
