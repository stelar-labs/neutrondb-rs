use std::convert::TryInto;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::str;
use crate::CustomError;

pub fn key(p: &Path, arg: &str) -> Result<Option<String>, Box<dyn Error>>  {

    let mut f = File::open(p)?;

    let mut list_start = [0; 9];

    f.read_exact(&mut list_start)?;

    if list_start[0] == 1 {

        let index_length: usize = u64::from_le_bytes(list_start[1..9].try_into().unwrap()) as usize;

        let mut index_buffer = vec![0; index_length];

        f.read_exact(&mut index_buffer)?;

        let mut i = 0;

        let mut index_list: Vec<(String, u64)> = Vec::new();

        while i < index_buffer.len() {

            let key_length_size: u8 = u8::from_le_bytes([index_buffer[i]]);

            i += 1;

            let mut key_length: usize = 0;

            match key_length_size {

                1 => {
                    key_length = u8::from_le_bytes([index_buffer[i]].try_into().unwrap()) as usize;
                    i += 1;
                },

                2 => {
                    key_length = u16::from_le_bytes(index_buffer[i..i + 2].try_into().unwrap()) as usize;
                    i += 2;
                },

                4 => {
                    key_length = u32::from_le_bytes(index_buffer[i..i + 4].try_into().unwrap()) as usize;
                    i += 4;
                },

                8 => {
                    key_length = u64::from_le_bytes(index_buffer[i..i + 8].try_into().unwrap()) as usize;
                    i += 8;
                },

                _ => ()

            }

            let key: String = str::from_utf8(index_buffer[i..i + key_length].try_into().unwrap()).unwrap().to_string();
            
            i += key_length;

            let value_index: u64 = u64::from_le_bytes(index_buffer[i..i + 8].try_into().unwrap());

            i += 8;

            index_list.push((key, value_index))

        }

        let index_query = index_list
            .iter()
            .find(|&x| &x.0 == arg);

        match index_query {
            Some(res) => {

                f.seek(SeekFrom::Start(res.1))?;

                let mut value_length_size_buffer = [0; 1];

                f.read_exact(&mut value_length_size_buffer)?;

                let value_length_size: u8 = u8::from_le_bytes(value_length_size_buffer);

                let mut value_length: usize = 0;

                match value_length_size {
                    
                    1 => {

                        let mut value_length_buffer = [0; 1];

                        f.read_exact(&mut value_length_buffer)?;

                        value_length = u8::from_le_bytes(value_length_buffer) as usize;

                    },

                    2 => {

                        let mut value_length_buffer = [0; 2];

                        f.read_exact(&mut value_length_buffer)?;

                        value_length = u16::from_le_bytes(value_length_buffer) as usize;

                    },

                    4 => {

                        let mut value_length_buffer = [0; 4];

                        f.read_exact(&mut value_length_buffer)?;

                        value_length = u32::from_le_bytes(value_length_buffer) as usize;

                    },

                    8 => {

                        let mut value_length_buffer = [0; 8];

                        f.read_exact(&mut value_length_buffer)?;

                        value_length = u64::from_le_bytes(value_length_buffer) as usize;

                    },

                    _ => ()
                }

                let mut value_buffer = vec![0; value_length];

                f.read_exact(&mut value_buffer)?;

                let value: String = str::from_utf8(&value_buffer).unwrap().to_string();

                Ok(Some(value))

            },
            None => Ok(None)
        }

    } else {
        Err(Box::new(CustomError("unsupport neutron file version".into())))?
    }

}