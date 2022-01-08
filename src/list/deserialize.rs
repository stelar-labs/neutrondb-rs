
use std::convert::TryInto;
use std::error::Error;
use std::str;

pub fn list(arg: &Vec<u8>) -> Result<Vec<(String, String)>, Box<dyn Error>> {

    let mut res: Vec<(String, String)> = Vec::new();

    if arg[0] == 1 {

        let index_size: usize = u64::from_le_bytes(arg[1..9].try_into().unwrap()) as usize;

        let index: Vec<u8> = arg[9..9 + index_size].to_vec();

        let mut i = 0;

        while i < index.len() {

            let key_length_size: u8 = u8::from_le_bytes([index[i]]);

            i += 1;
            
            let mut key_length: usize = 0;

            match key_length_size {

                1 => {
                    key_length = u8::from_le_bytes([index[i]].try_into().unwrap()) as usize;
                    i += 1;
                },

                2 => {
                    key_length = u16::from_le_bytes(index[i..i + 2].try_into().unwrap()) as usize;
                    i += 2;
                },

                4 => {
                    key_length = u32::from_le_bytes(index[i..i + 4].try_into().unwrap()) as usize;
                    i += 4;
                },

                8 => {
                    key_length = u64::from_le_bytes(index[i..i + 8].try_into().unwrap()) as usize;
                    i += 8;
                },

                _ => ()

            }

            let key: String = str::from_utf8(index[i..i + key_length].try_into().unwrap()).unwrap().to_string();
            
            i += key_length;

            let mut value_index: usize = u64::from_le_bytes(index[i..i + 8].try_into().unwrap()) as usize;

            i += 8;

            let value_length_size: u8 = u8::from_le_bytes([arg[value_index]]);

            value_index += 1;

            let mut value_length: usize = 0;

            match value_length_size {
                
                1 => {
                    value_length = u8::from_le_bytes([arg[value_index]].try_into().unwrap()) as usize;
                    value_index += 1;
                },

                2 => {
                    value_length = u16::from_le_bytes(arg[value_index..value_index + 2].try_into().unwrap()) as usize;
                    value_index += 2;
                },

                4 => {
                    value_length = u32::from_le_bytes(arg[value_index..value_index + 4].try_into().unwrap()) as usize;
                    value_index += 4;
                },

                8 => {
                    value_length = u64::from_le_bytes(arg[value_index..value_index + 8].try_into().unwrap()) as usize;
                    value_index += 8;
                },

                _ => ()
            }

            let value_bytes: Vec<u8> = arg[value_index..value_index + value_length].to_vec();

            let value: String = str::from_utf8(&value_bytes).unwrap().to_string();

            res.push((key, value))

        }

    }

    Ok(res)

}
