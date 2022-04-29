use std::convert::TryInto;
use std::{path::Path, error::Error};
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;
use astro_format::arrays;
use std::str;

pub fn search(key: &str, path: &Path) -> Result<String, Box<dyn Error>> {

    let mut file = File::open(path)?;
    
    let mut index_len_buffer = [0; 8];

    file.read_exact(&mut index_len_buffer)?;

    let index_len = usize::from_be_bytes(index_len_buffer);
    
    let mut bloom_len_buffer = [0; 8];
    
    file.read_exact(&mut bloom_len_buffer)?;

    let bloom_len = usize::from_be_bytes(bloom_len_buffer);

    let mut index_buffer = vec![0; index_len];

    file.read_exact(&mut index_buffer)?;

    let index_bytes = arrays::decode(&index_buffer).unwrap();

    let index: Vec<(String, u64)> = index_bytes
        .iter()
        .map(|x| {
            
            let key_seek = arrays::decode(x).unwrap();

            let key = str::from_utf8(&key_seek[0][..]).unwrap().to_string();

            let seek = u64::from_be_bytes(key_seek[1][..].try_into().unwrap());
            
            (key, seek)

        })
        .collect();
        
    match index.iter().position(|x| x.0 == key) {

        Some(i) => {

            let seek = index[i].1;

            let start = 16 + index_len as u64 + bloom_len as u64 + seek;

            let end = if i == index.len() - 1 {
                file.metadata()?.len()
            } else {
                start + index[i + 1].1
            };

            let value_len = (end - start) as usize;

            file.seek(SeekFrom::Start(start))?;

            let mut value_buffer = vec![0; value_len];
            
            file.read_exact(&mut value_buffer)?;

            let res = str::from_utf8(&value_buffer[..]).unwrap().to_string();

            Ok(res)

        },

        None => Err("Internal error!")?

    }

}