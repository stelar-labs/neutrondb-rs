
use std::error::Error;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use astro_notation::{ encode };

use crate::list;
use crate::Store;
use crate::Metadata;
use crate::store::bloom_filter;
use std::io::Write;
use std::fs::OpenOptions;
use std::path::Path;

pub fn run(store: &mut Store) -> Result<(), Box<dyn Error>> {
    
    let store_path = format!("./ndb/{}", store.name);

    for level in 1..=4 {

        let mut level_lists: Vec<&Metadata> = store.lists
            .iter()
            .filter(|x| x.level == level)
            .collect();

        let level_size: u64 = level_lists.iter().fold( 0, | acc, x | acc + x.size );

        let compaction_flag: bool = match level {
            
            1 => {
                if level_size > 10000000 { true } else { false }
            },

            2 => {
                if level_size > 100000000 { true } else { false }
            },

            3 => {
                if level_size > 1000000000 { true } else { false }
            },

            4 => {
                if level_size > 10000000000 { true } else { false }
            },

            _ => false

        };

        if compaction_flag {

            level_lists.sort_by_key(|x| x.name.to_string());

            let level_path = format!("{}/level_{}", &store_path, &level);

            let mut level_data: Vec<(String, String)> = level_lists
                .iter()
                .fold(vec![], | acc, x | {

                    let list_path = format!("{}/{}.neutron", &level_path, x.name);

                    let list_buffer = fs::read(&list_path).unwrap();

                    fs::remove_file(list_path).unwrap();

                    [acc, list::deserialize::list(&list_buffer).unwrap()].concat()

                });

            level_data.retain(|x| store.graves.contains(&x.0) == false);

            level_data.reverse();

            level_data.sort_by_key(|x| x.0.to_owned());

            level_data.dedup_by_key(|x| x.0.to_owned());

            let bloom_filter: Vec<u8> = level_data
                .iter()
                .fold(vec![0; 32], |acc, x| bloom_filter::insert(acc, &x.0));

            let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

            let next_level: u8 = level + 1;

            let next_level_path = format!("{}/level_{}/", &store_path, &next_level);

            fs::create_dir_all(&next_level_path)?;

            let compacted_path = format!("{}{}.neutron", &next_level_path, &current_time);

            let compacted_buffer: Vec<u8> = list::serialize::list(&level_data);

            fs::write(&compacted_path, &compacted_buffer)?;

            store.lists.retain(|x| x.level != level);

            let compaction_meta = fs::metadata(&compacted_path)?;

            let meta = Metadata{
                name: current_time.to_string(),
                level: next_level,
                size: compaction_meta.len(),
                bloom_filter: bloom_filter
            };

            store.lists.push(meta);

            let meta_path_str: String = format!("{}/meta", &store_path);

            let meta_path: &Path = Path::new(&meta_path_str);

            fs::remove_file(meta_path)?;

            let mut meta_file = OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open(meta_path)?;

            let meta_str: String = store.lists
                .iter()
                .fold(String::new(), | acc, x | {
                    format!("{}{} {} {} {}\n",
                    acc,
                    encode::str(&x.name),
                    encode::u8(&x.level),
                    encode::u64(&x.size),
                    encode::bytes(&x.bloom_filter))
                });

            write!(meta_file, "{}", &meta_str)?;

        }

    }

    Ok(())
}