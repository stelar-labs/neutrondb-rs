
use std::error::Error;
use std::str;

use crate::CustomError;

pub fn object(bytes: &Vec<u8>) -> Result<(String, String), Box<dyn Error>> {

    let key_length: usize = u8::from_le_bytes([bytes[0]; 1]) as usize;

    let key_string: String = str::from_utf8(&bytes[1..key_length]).unwrap().to_string();

    let vls: usize = u8::from_le_bytes([bytes[1 + key_length]; 1]) as usize;

    match vls {
        
        1 => {
            let vs = str::from_utf8(&bytes[3 + key_length..bytes.len()])?.to_string();
            Ok((key_string, vs))
        },

        2 => {
            let vs = str::from_utf8(&bytes[4 + key_length..bytes.len()])?.to_string();
            Ok((key_string, vs))
        },

        4 => {
            let vs = str::from_utf8(&bytes[6 + key_length..bytes.len()])?.to_string();
            Ok((key_string, vs))
        },

        8 => {
            let vs = str::from_utf8(&bytes[10 + key_length..bytes.len()])?.to_string();
            Ok((key_string, vs))
        },

        _ => {
            Err(Box::new(CustomError("value length size unknown".into())))
        }

    }

}

pub fn list(bytes: &Vec<u8>) -> Result<Vec<(String, String)>, Box<dyn Error>> {

    let mut list: Vec<(String, String)> = Vec::new();

    let mut i = 0;

    while i < bytes.len() {

        let key_length: usize = u8::from_le_bytes([bytes[i]; 1]) as usize;
        i += 1;

        let key_string: String = str::from_utf8(&bytes[i..i + key_length]).unwrap().to_string();
        i += key_length;

        let value_length_size: usize = u8::from_le_bytes([bytes[i]; 1]) as usize;
        i += 1;

        match value_length_size {

            1 => {

                let value_length: usize = u8::from_le_bytes([bytes[i]; 1]) as usize;
                i += 1;

                let value_string: String = str::from_utf8(&bytes[i..i + value_length])?.to_string();
                i += value_length;

                list.push((key_string, value_string))

            },

            2 => {

                let value_length: usize = u16::from_le_bytes([bytes[i], bytes[i + 1]]) as usize;
                i += 2;

                let value_string: String = str::from_utf8(&bytes[i..i + value_length])?.to_string();
                i += value_length;

                list.push((key_string, value_string))

            },

            4 => {

                let value_length: usize = u32::from_le_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]) as usize;
                i += 4;

                let value_string: String = str::from_utf8(&bytes[i..i + value_length])?.to_string();
                i += value_length;

                list.push((key_string, value_string))

            },

            8 => {

                let value_length: usize = u64::from_le_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3], bytes[i + 4], bytes[i + 5], bytes[i + 6], bytes[i + 7]]) as usize;
                i += 8;

                let value_string: String = str::from_utf8(&bytes[i..i + value_length])?.to_string();
                i += value_length;

                list.push((key_string, value_string))

            },

            _ => {
                Err(Box::new(CustomError("value length size unknown".into())))?
            }

        }

    }

    Ok(list)

}
