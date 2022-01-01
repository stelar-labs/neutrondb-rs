
use std::error::Error;
use std::str;

use crate::CustomError;

pub fn object(bytes: Vec<u8>) -> Result<(String, String), Box<dyn Error>> {

    let key_length_size: u8 = u8::from_le_bytes([bytes[0]]);

    match key_length_size {

        1 => {
            let key_length: usize = u8::from_le_bytes([bytes[1]]) as usize;
            let key_length_end = key_length + 2;
            let key_string = str::from_utf8(&bytes[2..key_length_end]).unwrap().to_string();
            let value_string = str::from_utf8(&bytes[key_length_end..]).unwrap().to_string();
            Ok((key_string, value_string))
        },

        2 => {
            let key_length: usize = u16::from_le_bytes([bytes[1], bytes[2]]) as usize;
            let key_length_end = key_length + 3;
            let key_string = str::from_utf8(&bytes[3..key_length_end]).unwrap().to_string();
            let value_string = str::from_utf8(&bytes[key_length_end..]).unwrap().to_string();
            Ok((key_string, value_string))
        },

        4 => {
            let key_length: usize = u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]) as usize;
            let key_length_end = key_length + 5;
            let key_string = str::from_utf8(&bytes[5..key_length_end]).unwrap().to_string();
            let value_string = str::from_utf8(&bytes[key_length_end..]).unwrap().to_string();
            Ok((key_string, value_string))
        },

        8 => {
            let key_length: usize = u64::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8]]) as usize;
            let key_length_end = key_length + 9;
            let key_string = str::from_utf8(&bytes[9..key_length_end]).unwrap().to_string();
            let value_string = str::from_utf8(&bytes[key_length_end..]).unwrap().to_string();
            Ok((key_string, value_string))
        },

        _ => {
            Err(Box::new(CustomError("key length size unknown".into())))
        }

    }

}

pub fn list(bytes: &Vec<u8>) -> Result<Vec<(String, String)>, Box<dyn Error>> {

    let mut res: Vec<(String, String)> = Vec::new();

    let mut i: usize = 0;

    while i < bytes.len() {

        let object_length_size: u8 = u8::from_le_bytes([bytes[i]]);

        match object_length_size {
            1 => {
                let object_length: usize = u8::from_le_bytes([bytes[i + 1]]) as usize;
                let object_bytes: Vec<u8> = bytes[i..i+object_length].to_vec();
                let key_value: (String, String) = object(object_bytes).unwrap();
                res.push(key_value);
                i += 1 + object_length
            },
            2 => {
                let object_length: usize = u8::from_le_bytes([bytes[i + 1]]) as usize;
                let object_bytes: Vec<u8> = bytes[i..i+object_length].to_vec();
                let key_value: (String, String) = object(object_bytes).unwrap();
                res.push(key_value);
                i += 2 + object_length
            },
            4 => {
                let object_length: usize = u8::from_le_bytes([bytes[i + 1]]) as usize;
                let object_bytes: Vec<u8> = bytes[i..i+object_length].to_vec();
                let key_value: (String, String) = object(object_bytes).unwrap();
                res.push(key_value);
                i += 4 + object_length
            },
            8 => {
                let object_length: usize = u8::from_le_bytes([bytes[i + 1]]) as usize;
                let object_bytes: Vec<u8> = bytes[i..i+object_length].to_vec();
                let key_value: (String, String) = object(object_bytes).unwrap();
                res.push(key_value);
                i += 8 + object_length
            },
            _ => ()
        }

    }

    Ok(res)

}
