
pub fn list(arg: &Vec<(String, String)>) -> Vec<u8> {

    let mut index: Vec<(Vec<u8>, u64)> = Vec::new();

    let mut val_res: Vec<u8> = Vec::new();

    for obj in arg {

        let mut key_res: Vec<u8> = Vec::new();

        let key_bytes: Vec<u8> = obj.0.to_string().into_bytes();

        let key_length: usize = key_bytes.len();

        if key_length < 256 {

            1_u8.to_le_bytes().to_vec().iter().for_each(|&x| key_res.push(x));
    
            let key_length_size: u8 = key_length as u8;
    
            key_length_size.to_le_bytes().to_vec().iter().for_each(|&x| key_res.push(x))
        
        } else if key_length < 65536 {
    
            2_u8.to_le_bytes().to_vec().iter().for_each(|&x| key_res.push(x));
    
            let key_length_size: u16 = key_length as u16;
    
            key_length_size.to_le_bytes().to_vec().iter().for_each(|&x| key_res.push(x))
    
        } else if key_length < 4294967296 {
    
            4_u32.to_le_bytes().to_vec().iter().for_each(|&x| key_res.push(x));
    
            let key_length_size: u32 = key_length as u32;
    
            key_length_size.to_le_bytes().to_vec().iter().for_each(|&x| key_res.push(x))
    
        } else {
    
            8_u64.to_le_bytes().to_vec().iter().for_each(|&x| key_res.push(x));
    
            let key_length_size: u64 = key_length as u64;
    
            key_length_size.to_le_bytes().to_vec().iter().for_each(|&x| key_res.push(x))
    
        }
        
        key_bytes.iter().for_each(|&x| key_res.push(x));

        index.push((key_res, val_res.len() as u64));

        let val_bytes: Vec<u8> = obj.1.to_string().into_bytes();

        let val_length: usize = val_bytes.len();

        if val_length < 256 {

            1_u8.to_le_bytes().to_vec().iter().for_each(|&x| val_res.push(x));
    
            let val_length_size: u8 = val_length as u8;
    
            val_length_size.to_le_bytes().to_vec().iter().for_each(|&x| val_res.push(x))
        
        } else if val_length < 65536 {
    
            2_u8.to_le_bytes().to_vec().iter().for_each(|&x| val_res.push(x));
    
            let val_length_size: u16 = val_length as u16;
    
            val_length_size.to_le_bytes().to_vec().iter().for_each(|&x| val_res.push(x))
    
        } else if val_length < 4294967296 {
    
            4_u32.to_le_bytes().to_vec().iter().for_each(|&x| val_res.push(x));
    
            let val_length_size: u32 = val_length as u32;
    
            val_length_size.to_le_bytes().to_vec().iter().for_each(|&x| val_res.push(x))
    
        } else {
    
            8_u64.to_le_bytes().to_vec().iter().for_each(|&x| val_res.push(x));
    
            let val_length_size: u64 = val_length as u64;
    
            val_length_size.to_le_bytes().to_vec().iter().for_each(|&x| val_res.push(x))
    
        }

        val_bytes.iter().for_each(|&x| val_res.push(x))

    }

    let mut index_size: u64 = index.iter().fold(0, | acc, x | acc + (x.0.len() as u64 + 8));

    let list_start: Vec<u8> = [vec![1], index_size.to_le_bytes().to_vec()].concat();

    index_size += 9;

    let list_header: Vec<u8> = index.iter().fold(list_start, | acc, x | {

        let key_index: u64 = x.1 + index_size;

        [acc, x.0.to_owned(), key_index.to_le_bytes().to_vec()].concat()

    });

    [list_header, val_res].concat()

}