
pub fn object(key: &str, value: &str) -> Vec<u8> {
    
    let key_bytes: Vec<u8> = key.to_string().into_bytes();

    let key_length: u8 = key_bytes.len() as u8;

    let value_bytes: Vec<u8> = value.to_string().into_bytes();

    let value_length: u64 = value_bytes.len() as u64;

    if value_length < 256 {

        let value_length_size: Vec<u8> = 1_u8.to_le_bytes().to_vec();

        let u8_value_length: u8 = value_length as u8;

        [vec![key_length], key_bytes, value_length_size, u8_value_length.to_le_bytes().to_vec(), value_bytes].concat()
        
    } else if value_length < 65536 {

        let value_length_size: Vec<u8> = 2_u8.to_le_bytes().to_vec();

        let u16_value_length: u16 = value_length as u16;

        [vec![key_length], key_bytes, value_length_size, u16_value_length.to_le_bytes().to_vec(), value_bytes].concat()

    } else if value_length < 4294967296 {

        let value_length_size: Vec<u8> = 4_u8.to_le_bytes().to_vec();

        let u32_value_length: u32 = value_length as u32;

        [vec![key_length], key_bytes, value_length_size, u32_value_length.to_le_bytes().to_vec(), value_bytes].concat()

    } else {

        let value_length_size: Vec<u8> = 8_u8.to_le_bytes().to_vec();

        let u64_value_length: u64 = value_length as u64;

        [vec![key_length], key_bytes, value_length_size, u64_value_length.to_le_bytes().to_vec(), value_bytes].concat()

    }

}

pub fn list(objects: &Vec<(String, String)>) -> Vec<u8> {

    let res: Vec<Vec<u8>> = objects
        .iter()
        .map(|x| object(&x.0, &x.1))
        .collect();

    res.concat()

}