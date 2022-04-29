use std::convert::TryInto;
use std::error::Error;
use astro_format::arrays;
use std::str;

pub fn fetch(buffer: &Vec<u8>) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    
    let index_buffer: [u8; 8] = buffer[..8].try_into().unwrap();

    let index_len = usize::from_be_bytes(index_buffer);

    let bloom_buffer: [u8; 8] = buffer[8..16].try_into().unwrap();
    
    let bloom_len = usize::from_be_bytes(bloom_buffer);

    let index_end = 16 + index_len;

    let index_buffer = &buffer[16..index_end].to_vec();

    let index_bytes = arrays::decode(&index_buffer).unwrap();

    let keys_seeks: Vec<(String, usize)> = index_bytes
        .iter()
        .map(|x| {
            
            let key_seek = arrays::decode(x).unwrap();

            let key = str::from_utf8(&key_seek[0][..]).unwrap().to_string();

            let seek = usize::from_be_bytes(key_seek[1][..].try_into().unwrap());
            
            (key, seek)

        })
        .collect();

    let value_start = index_end + bloom_len;

    let res = keys_seeks
        .iter()
        .enumerate()
        .map(|(i, x)| {

            let start = value_start + x.1;

            let end = if i == keys_seeks.len() - 1 {
                buffer.len()
            } else {
                value_start + keys_seeks[i + 1].1
            };

            let value = str::from_utf8(&buffer[start..end]).unwrap().to_string();

            (x.0.clone(), value)

        })
        .collect();

    Ok(res)  

}