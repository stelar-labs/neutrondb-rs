use fides::BloomFilter;
use crate::{Table, Store};
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

impl<K,V> Store<K,V> {
    
    pub fn compaction(&mut self) -> Result<(), Box<dyn Error>>
    where
    K: std::clone::Clone + std::cmp::PartialEq + std::cmp::Ord + Into<Vec<u8>> + TryFrom<Vec<u8>>,
    V: std::clone::Clone + Into<Vec<u8>> + TryFrom<Vec<u8>>
    {

        for level in 1..=10 {

            let tables: Vec<&Table> = self.tables.iter().filter(|x| x.level == level).collect();

            let level_size = tables.iter().fold(0, |acc, x| acc + x.size);

            if level_size > (10_u64.pow(level as u32) * 1000000) {

                let mut readers = vec![];
                
                for table in &tables {
                    
                    let path = format!("{}/levels/{}/{}", &self.directory, table.level, table.name);

                    let file = File::open(path)?;

                    let reader = BufReader::new(file);

                    let mut lines = reader.lines();

                    lines.next();

                    lines.next();

                    readers.push(lines)

                }

                let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

                let next_level_path = format!("{}/levels/{}", &self.directory, level + 1);

                fs::create_dir_all(&next_level_path)?;

                let compacted_path_str = format!("{}/{}.neutron", &next_level_path, &current_time);

                let compacted_path = Path::new(&compacted_path_str);

                let mut compacted_file = OpenOptions::new().append(true).create(true).open(compacted_path)?;

                let mut key_vals: Vec<Option<(K,V)>> = vec![None; readers.len()];
                
                for i in 0..readers.len() {

                    match readers[i].next() {
                        Some(res) => {
                            match key_and_val(&res?) {
                                Ok(r) => key_vals[i] = Some(r),
                                Err(_) => ()
                            }
                        },
                        None => ()
                    }
                
                }

                let obj_count: u64 = tables.iter().map(|x| x.count).sum();

                let mut bloom_filter = BloomFilter::new(obj_count.try_into()?);

                write!(compacted_file, "neutrondb table version 1.0\n\n")?;

                let mut key_val_none_check = key_vals
                    .iter()
                    .all(|x| match x {
                        Some(_) => false,
                        None => true,
                    });

                while !key_val_none_check {

                    let min = key_vals
                        .iter()
                        .filter(|&x| {
                            match x {
                                Some(_) => true,
                                None => false
                            }
                        })
                        .map(|x| x.clone().unwrap().0.clone())
                        .min()
                        .unwrap();

                    let mut min_idx = key_vals
                        .iter()
                        .position(|x| match x {
                            Some(res) => res.0 == min,
                            None => false
                        });

                    let mut position = min_idx.unwrap();

                    let k_bytes: Vec<u8> = key_vals[position].as_ref().unwrap().0.clone().into();

                    bloom_filter.insert(&k_bytes);

                    let v_bytes: Vec<u8> = key_vals[position].as_ref().unwrap().1.clone().into();
                    
                    write!(compacted_file, "{} {}\n", hex::encode(&k_bytes), hex::encode(&v_bytes))?;

                    while min_idx != None {

                        match readers[position].next() {
                            Some(res) => {
                                match key_and_val(&res?) {
                                    Ok(r) => key_vals[position] = Some(r),
                                    Err(_) => key_vals[position] = None
                                }
                            },
                            None => ()
                        }

                        min_idx = key_vals
                            .iter()
                            .position(|x| match x {
                                Some(res) => res.0 == min,
                                None => false
                            });

                        match min_idx {
                            Some(res) => position = res,
                            None => ()
                        }
                        
                    }
                    
                    key_val_none_check = key_vals
                        .iter()
                        .all(|x| match x {
                            Some(_) => false,
                            None => true,
                        });

                }

                let bloom_filter_bytes: Vec<u8> = (&bloom_filter).into();

                write!(
                    compacted_file,
                    "\n{}\n",
                    hex::encode(&bloom_filter_bytes)
                )?;

                for table in tables {

                    let table_path = format!(
                        "{}/levels/{}/{}",
                        &self.directory,
                        &table.level,
                        &table.name
                    );

                    fs::remove_file(table_path)?;

                }

                self.tables.retain(|x| x.level != level);

                let table = Table {
                    bloom_filter,
                    level: level + 1,
                    name: format!("{}.neutron", current_time),
                    size: compacted_file.metadata()?.len(),
                    count: (BufReader::new(File::open(compacted_path)?).lines().count() - 4) as u64
                };

                self.tables.push(table);

                self.tables.sort_by_key(|k| k.name.clone());

                self.tables.reverse()

            }

        }

        Ok(())

    }

}

fn key_and_val<K,V>(val: &str) -> Result<(K,V), Box<dyn Error>> 
where K: TryFrom<Vec<u8>>, V: TryFrom<Vec<u8>>
{
    let spt_val: Vec<&str> = val.split(" ").collect();
    if spt_val.len() == 2 {
        let k_bytes_hex = spt_val[0];
        let k_bytes = hex::decode(k_bytes_hex)?;
        match K::try_from(k_bytes) {
            Ok(k) => {
                let v_bytes_hex = spt_val[1];
                let v_bytes = hex::decode(v_bytes_hex)?;
                match V::try_from(v_bytes) {
                    Ok(v) => {
                        Ok((k,v))
                    },
                    Err(_) => todo!(),
                }
            },
            Err(_) => todo!(),
        }
    } else {
        Err("Not Supported!")?
    }
}
