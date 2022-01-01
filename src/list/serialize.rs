
pub fn object(key: &str, value: &str) -> Vec<u8> {

    let mut res: Vec<u8> = Vec::new();
    
    let key_bytes: Vec<u8> = key.to_string().into_bytes();

    let key_length: usize = key_bytes.len();

    if key_length < 256 {

        res = [res, 1_u8.to_le_bytes().to_vec()].concat();

        let key_length_size: u8 = key_length as u8;

        res = [res, key_length_size.to_le_bytes().to_vec()].concat();
    
    } else if key_length < 65536 {

        res = [res, 2_u8.to_le_bytes().to_vec()].concat();

        let key_length_size: u16 = key_length as u16;

        res = [res, key_length_size.to_le_bytes().to_vec()].concat();

    } else if key_length < 4294967296 {

        res = [res, 4_u32.to_le_bytes().to_vec()].concat();

        let key_length_size: u32 = key_length as u32;

        res = [res, key_length_size.to_le_bytes().to_vec()].concat();

    } else {

        res = [res, 8_u64.to_le_bytes().to_vec()].concat();

        let key_length_size: u64 = key_length as u64;

        res = [res, key_length_size.to_le_bytes().to_vec()].concat();

    }

    let value_bytes: Vec<u8> = value.to_string().into_bytes();

    res = [res, key_bytes, value_bytes].concat();

    res

}

pub fn list(objects: &Vec<(String, String)>) -> Vec<u8> {

    let mut res: Vec<u8> = Vec::new();

    objects
        .iter()
        .for_each(|x| {

            let object_bytes: Vec<u8> = object(&x.0, &x.1);

            let object_length: usize = object_bytes.len();

            if object_length < 256 {

                res = [res.clone(), 1_u8.to_le_bytes().to_vec()].concat();

                let object_length_size: u8 = object_length as u8;

                res = [res.clone(), object_length_size.to_le_bytes().to_vec()].concat()

            } else if object_length < 65536 {

                res = [res.clone(), 2_u8.to_le_bytes().to_vec()].concat();

                let object_length_size: u16 = object_length as u16;

                res = [res.clone(), object_length_size.to_le_bytes().to_vec()].concat()

            } else if object_length < 4294967296 {

                res = [res.clone(), 4_u32.to_le_bytes().to_vec()].concat();

                let object_length_size: u32 = object_length as u32;

                res = [res.clone(), object_length_size.to_le_bytes().to_vec()].concat()

            } else {
                
                res = [res.clone(), 8_u64.to_le_bytes().to_vec()].concat();

                let object_length_size: u64 = object_length as u64;

                res = [res.clone(), object_length_size.to_le_bytes().to_vec()].concat()

            }

            res = [res.clone(), object_bytes].concat();

        });

    res

}