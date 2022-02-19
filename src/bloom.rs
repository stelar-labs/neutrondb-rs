
use blake3;
use sha2::{Sha256, Sha512, Digest};
use sha3::{Sha3_256, Sha3_512};

pub fn insert(filter: Vec<u8>, key: &str) -> Vec<u8> {

    let hash_results = vec![h1(&key), h2(&key), h3(&key), h4(&key), h5(&key)];

    let result = hash_results.iter()
        .fold(filter, |acc, x| toggle(acc, x));

    return result.to_owned()

}

pub fn lookup(filter: &Vec<u8>, key: &str) -> bool {

    let hash_results = vec![h1(&key), h2(&key), h3(&key), h4(&key), h5(&key)];

    let result = hash_results.iter()
        .all(|x| is_toggled(filter, x));

    return result

}

fn toggle(filter: Vec<u8>, hash: &u8) -> Vec<u8> {

    let byte_index: u8 = hash/8;
    let bit_index: u8 = hash%8;

    let mut byte = filter[byte_index as usize];

    match bit_index {
        0 => byte |= 0b1000_0000,
        1 => byte |= 0b0100_0000,
        2 => byte |= 0b0010_0000,
        3 => byte |= 0b0001_0000,
        4 => byte |= 0b0000_1000,
        5 => byte |= 0b0000_0100,
        6 => byte |= 0b0000_0010,
        _ => byte |= 0b0000_0001,
    }

    let mut result = filter;

    result[byte_index as usize] = byte;

    return result

}

fn is_toggled(filter: &Vec<u8>, hash: &u8) -> bool {

    let byte_index: u8 = hash/8;
    let bit_index: u8 = hash%8;

    let byte = filter[byte_index as usize];

    let mut new_byte = byte;

    match bit_index {
        0 => new_byte |= 0b1000_0000,
        1 => new_byte |= 0b0100_0000,
        2 => new_byte |= 0b0010_0000,
        3 => new_byte |= 0b0001_0000,
        4 => new_byte |= 0b0000_1000,
        5 => new_byte |= 0b0000_0100,
        6 => new_byte |= 0b0000_0010,
        _ => new_byte |= 0b0000_0001,
    }

    if byte == new_byte {
        return true
    } else {
        return false
    }

}

fn h1(key: &str) -> u8 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&key.to_owned().into_bytes());
    let hash = hasher.finalize();
    let integer = u8::from_le(hash.as_bytes()[0]);
    return integer
}

fn h2(key: &str) -> u8 {
    let mut hasher = Sha256::new();
    hasher.update(key);
    let hash = hasher.finalize();
    let integer = u8::from_le(hash[0]);
    return integer
}

fn h3(key: &str) -> u8 {
    let mut hasher = Sha512::new();
    hasher.update(key);
    let hash = hasher.finalize();
    let integer = u8::from_le(hash[0]);
    return integer
}

fn h4(key: &str) -> u8 {
    let mut hasher = Sha3_256::new();
    hasher.update(key);
    let hash = hasher.finalize();
    let integer = u8::from_le(hash[0]);
    return integer
}

fn h5(key: &str) -> u8 {
    let mut hasher = Sha3_512::new();
    hasher.update(key);
    let hash = hasher.finalize();
    let integer = u8::from_le(hash[0]);
    return integer
}
