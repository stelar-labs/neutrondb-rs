use astro_notation::{encode};
use crate::Metadata;
use crate::list;
use crate::Store;
use crate::store::bloom_filter;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Write;

pub fn run(store: &mut Store) -> Result<(), Box<dyn Error>> {

    let store_path = format!("./ndb/{}", store.name);

    let level1_path = format!("{}/level_1/", &store_path);

    if !Path::new(&level1_path).is_dir() { fs::create_dir(&level1_path)? }

    store.cache.reverse();
    store.cache.sort_by_key(|x| x.0.to_owned());
    store.cache.dedup_by_key(|x| x.0.to_owned());

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    let new_level1_path: String = format!("{}{}.neutron", &level1_path, &current_time);

    let new_level1_buffer: Vec<u8> = list::serialize::list(&store.cache);

    fs::write(&new_level1_path, &new_level1_buffer)?;

    let list_bloom_filter = store.cache
        .iter()
        .fold(vec![0; 32], |acc, x| bloom_filter::insert(acc, &x.0));

    let flush_meta = fs::metadata(&new_level1_path)?;

    let meta = Metadata{
        name: current_time.to_string(),
        level: 1,
        size: flush_meta.len() as u64,
        bloom_filter: list_bloom_filter
    };

    let meta_put: String = format!(
        "{} {} {} {}\n",
        encode::str(&meta.name),
        encode::u8(&1),
        encode::u64(&meta.size),
        encode::bytes(&meta.bloom_filter)
    );

    store.lists.push(meta);

    let meta_path_str: String = format!("{}/meta", &store_path);

    let meta_path: &Path = Path::new(&meta_path_str);

    let mut meta_file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(meta_path)?;

    write!(meta_file, "{}", &meta_put)?;

    let graves_path_str: String = format!("{}/graves", &store_path);

    let graves_path: &Path = Path::new(&graves_path_str);

    if graves_path.is_file() {

        fs::remove_file(graves_path)?;

        let mut graves_file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(graves_path)?;

        for grave in store.graves.clone() {

            let graves_put: String = format!("{}", encode::str(&grave));

            write!(graves_file, "{}", &graves_put)?;

        }

    }

    Ok(())

}