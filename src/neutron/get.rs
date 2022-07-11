use std::convert::TryInto;
use std::error::Error;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;
use astro_format::arrays;
use std::str;

pub fn get(key: &str, path: &str) -> Result<String, Box<dyn Error>> {

    let mut file = File::open(path)?;
    
    let mut bloom_buffer_len = [0; 8];

    file.read_exact(&mut bloom_buffer_len)?;

    let key_buffer_file_index = u64::from_le_bytes(bloom_buffer_len) + 8;

    file.seek(SeekFrom::Start(key_buffer_file_index))?;

    let mut key_buffer_len = [0; 8];

    file.read_exact(&mut key_buffer_len)?;

    let key_buffer_size = usize::from_be_bytes(key_buffer_len);

    let mut key_buffer = vec![0; key_buffer_size];

    file.read_exact(&mut key_buffer)?;

    let key_bytes = arrays::decode(&key_buffer)?;

    let keys: Vec<&str> = key_bytes.iter().map(|x| str::from_utf8(x).unwrap()).collect();

    match keys.iter().position(|&x| x == key) {

        Some(i) => {

            let index_buffer_file_index = key_buffer_file_index + key_buffer_size as u64;

            file.seek(SeekFrom::Start(index_buffer_file_index))?;

            let mut index_buffer_size_len = [0; 8];

            file.read_exact(&mut index_buffer_size_len)?;

            let index_buffer_size = usize::from_le_bytes(index_buffer_size_len);

            let mut index_buffer = vec![0; index_buffer_size];

            file.read_exact(&mut index_buffer)?;

            let index_bytes = arrays::decode(&index_buffer)?;

            let start = usize::from_le_bytes(index_bytes[i].try_into()?);

            let end = usize::from_le_bytes(index_bytes[i + 1].try_into()?);

            let value_buffer_size = end - start;

            let value_buffer_file_index = index_buffer_file_index + index_buffer_size as u64;

            file.seek(SeekFrom::Start(value_buffer_file_index))?;

            let mut value_buffer = vec![0; value_buffer_size];

            file.read_exact(&mut value_buffer)?;

            let value = String::from_utf8(value_buffer)?;

            Ok(value)

        },

        None => Err("Key not found!")?

    }

}