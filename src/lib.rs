use std::fs::File;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Write;

use astro_notation::{encode, decode};

mod list;
mod bloom;

#[derive(Clone, Debug)]
struct Table {
    name: String,
    level: u8,
    bloom_filter: Vec<u8>
}

#[derive(Debug)]
pub struct Store {
    directory: String,
    logs: File,
    cache: Vec<(String, String)>,
    graves: Vec<String>,
    meta: Vec<Table>
}

impl Store {

    pub fn connect(name: &str) -> Store {

        let directory = format!("./ndb/{}", name);

        let path = Path::new(&directory);

        fs::create_dir_all(&path).unwrap();
    
        let logs_path = format!("{}/logs", &directory);

        let logs: File = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(Path::new(&logs_path))
            .unwrap();

        let mut output: Store = Store {
            directory: directory.clone(),
            logs: logs,
            cache: Vec::new(),
            graves: Vec::new(),
            meta: Vec::new()
        };
        
        if Path::new(&format!("{}/graves", &directory)).is_file() {
            for line in BufReader::new(File::open(format!("{}/graves", &directory)).unwrap()).lines() {
                output.graves.push(decode::as_str(&line.unwrap()))
            }
        }

        if Path::new(&format!("{}/meta", &directory)).is_file() {
            
            for line in BufReader::new(File::open(format!("{}/meta", &directory)).unwrap()).lines() {

                let line = line.unwrap();
                let split: Vec<&str> = line.split(' ').collect();
                
                let table: Table = Table {
                    name: decode::as_str(split[0]),
                    level: decode::as_u8(split[1]),
                    bloom_filter: decode::as_bytes(split[2])
                };
    
                output.meta.push(table)
    
            }
    
        }

        if Path::new(&logs_path).is_file() {
            
            for line in BufReader::new(File::open(logs_path).unwrap()).lines() {


                let line = line.unwrap();
                let split: Vec<&str> = line.split(' ').collect();
    
                let log_type: u8 = decode::as_u8(split[0]);
    
                match log_type {
                    1 => {
                        
                        let key: String = str::from_utf8(&decode::as_bytes(split[1])).unwrap().to_string();
    
                        let value: String = str::from_utf8(&decode::as_bytes(split[2])).unwrap().to_string();
    
                        output.cache.push((key.clone(), value));
                        
                        match output.graves.iter().find(|&x| x == &key) {
                            Some(_) => output.graves.retain(|x| x != &key),
                            None => ()
                        }
    
                    },
                    2 => output.graves.push(decode::as_str(split[1])),
                    _ => ()
    
                }
    
            }
    
        }

        output

    }

    pub fn put(&mut self, key: &str, value: &str) {
        
        self.cache.push((key.to_string(), value.to_string()));

        let logs_put: String = format!(
            "0x01 {} {}\n",
            encode::bytes(&key.to_string().into_bytes()),
            encode::bytes(&value.to_string().into_bytes())
        );

        write!(self.logs, "{}", &logs_put).unwrap();

        if self.logs.metadata().unwrap().len() > 2097152 {

            self.flush();
        
            fs::remove_file(Path::new(&format!("{}/logs", &self.directory))).unwrap();

            self.logs = OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open(Path::new(&format!("{}/logs", &self.directory)))
                .unwrap();

            self.cache.clear();

            self.compaction();

        }
    
        match self.graves.iter().find(|&x| x == key) {
            Some(_) => self.graves.retain(|x| x != key),
            None => ()
        }

    }

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
                
                let meta_path: &Path = Path::new(&format!("{}/meta", &self.directory));

                fs::remove_file(meta_path).unwrap();

                self.meta.push(table);
                
                let mut meta_file = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .create(true)
                    .open(meta_path)
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

    pub fn get(self, key: &str) -> Option<String> {
        
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

    pub fn get_all(self) -> Option<Vec<(String, String)>> {

        let mut res: Vec<(String, String)> = Vec::new();

        for table in &self.meta {

            let buf = fs::read(format!("{}/{}.neutron", &self.directory, table.name)).unwrap();

            res = [res, list::deserialize::list(&buf).unwrap()].concat();

        }

        res = [res, self.cache].concat();

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

    pub fn delete(&mut self, key: &str) {
        
        match self.graves.iter().find(|x| x == &key) {
            
            Some(_) => (),
            
            None => {

                self.graves.push(key.to_string());

                let logs_path_str: String = format!("{}/logs", &self.directory);

                let logs_path: &Path = Path::new(&logs_path_str); 

                let logs_put: String = format!("0x02 {}\n", encode::str(key));

                let mut logs_file = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .create(true)
                    .open(logs_path)
                    .unwrap();

                write!(logs_file, "{}", &logs_put).unwrap();

            }

        }

    }

}