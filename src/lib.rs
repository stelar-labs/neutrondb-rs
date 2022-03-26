
mod bloom;
mod list;
mod store;
use astro_notation::encode;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
struct Table {
    name: String,
    level: u8,
    bloom_filter: Vec<u8>
}

#[derive(Debug)]
pub struct Store {
    directory: String,
    cache: Vec<(String, String)>,
    graves: Vec<String>,
    meta: Vec<Table>
}

impl Store {

    fn flush(&mut self) {

        let level1_path = format!("{}/tables/level_1/", self.directory);

        if !Path::new(&level1_path).is_dir() {
            fs::create_dir_all(&level1_path).unwrap()
        }

        self.cache.reverse();
        self.cache.sort_by_key(|x| x.0.to_owned());
        self.cache.dedup_by_key(|x| x.0.to_owned());

        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        let table: Table = Table {
            name: current_time.to_string(),
            level: 1_u8,
            bloom_filter: self.cache.iter().fold(vec![0; 32], |acc, x| bloom::insert(acc, &x.0))
        };

        fs::write(format!("{}{}.neutron", &level1_path, &current_time), &list::serialize::list(&self.cache)).unwrap();

        let meta_put: String = format!(
            "{} {} {}\n",
            encode::str(&table.name),
            encode::u8(&table.level),
            encode::bytes(&table.bloom_filter)
        );

        self.meta.push(table);
        
        let mut meta_file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(Path::new(&format!("{}/meta", &self.directory)))
            .unwrap();

        write!(meta_file, "{}", &meta_put).unwrap();

        if Path::new(&format!("{}/graves", &self.directory)).is_file() {

            fs::remove_file(Path::new(&format!("{}/graves", &self.directory))).unwrap();

            let mut graves_file = OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open(Path::new(&format!("{}/graves", &self.directory)))
                .unwrap();

            for grave in self.graves.clone() {
                write!(graves_file, "{}", format!("{}", encode::str(&grave))).unwrap()
            }

        }

    }
    
    fn compaction(&mut self) {

        for level in 1..=4 {

            let mut tables: Vec<&Table> = self.meta.iter().filter(|x| x.level == level).collect();

            if tables.len() > 9 {

                tables.sort_by_key(|x| x.name.to_string());

                let level_path = format!("{}/tables/level_{}", &self.directory, &level);

                let mut level_data: Vec<(String, String)> = tables
                    .iter()
                    .fold(vec![], | acc, x | {

                        let table_path = format!("{}/{}.neutron", &level_path, x.name);

                        let table_buffer = fs::read(&table_path).unwrap();

                        fs::remove_file(table_path).unwrap();

                        [acc, list::deserialize::list(&table_buffer).unwrap()].concat()

                    });

                level_data.retain(|x| self.graves.contains(&x.0) == false);

                level_data.reverse();

                level_data.sort_by_key(|x| x.0.to_owned());

                level_data.dedup_by_key(|x| x.0.to_owned());

                let bloom_filter: Vec<u8> = level_data
                    .iter()
                    .fold(vec![0; 32], |acc, x| bloom::insert(acc, &x.0));

                let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

                let next_level: u8 = level + 1;

                let next_level_path = format!("{}/tables/level_{}/", &self.directory, &next_level);

                fs::create_dir_all(&next_level_path).unwrap();

                let compacted_path = format!("{}{}.neutron", &next_level_path, &current_time);

                let compacted_buffer: Vec<u8> = list::serialize::list(&level_data);

                fs::write(&compacted_path, &compacted_buffer).unwrap();

                self.meta.retain(|x| x.level != level);

                let table = Table{
                    name: current_time.to_string(),
                    level: next_level,
                    bloom_filter: bloom_filter
                };
                
                let meta_path = format!("{}/meta", &self.directory);

                fs::remove_file(Path::new(&meta_path)).unwrap();

                self.meta.push(table);
                
                let mut meta_file = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .create(true)
                    .open(Path::new(&meta_path))
                    .unwrap();
        
                let meta_str: String = self.meta
                    .iter()
                    .fold(String::new(), | acc, x | {
                        format!("{}{} {} {}\n",
                        acc,
                        encode::str(&x.name),
                        encode::u8(&x.level),
                        encode::bytes(&x.bloom_filter))
                    });

                write!(meta_file, "{}", &meta_str).unwrap();

            }

        }

    }

}