use astro_format;
use std::collections::BTreeMap;

pub fn run<K,V>(bloom: Vec<u8>, key_values: &BTreeMap<K,V>) -> Vec<u8>

    where
    
        K: Clone + Into<Vec<u8>>,
        
        V: Clone + Into<Vec<u8>>
        
            {

                let mut keys = Vec::new();

                let mut values = Vec::new();

                let mut index = 0_u64;

                for (key, value) in key_values.iter() {

                    let key_bytes: Vec<u8> = key.clone().into();

                    let index_bytes = index.to_le_bytes().to_vec();

                    let key_index_array: [&[u8];2] = [&key_bytes, &index_bytes];

                    let key_index_bytes = astro_format::encode(&key_index_array);

                    keys.push(key_index_bytes);

                    let value_bytes: Vec<u8> = value.clone().into();

                    index += value_bytes.len() as u64;

                    values.push(value_bytes);
                    
                }

                let keys_slices = keys.iter().map(|x| &x[..]).collect::<Vec<&[u8]>>();

                let keys_buffer = astro_format::encode(&keys_slices);

                let value_buffer = values.concat();

                [
                    (bloom.len() as u64).to_le_bytes().to_vec(), bloom,
                    (keys_buffer.len() as u64).to_le_bytes().to_vec(), keys_buffer,
                    value_buffer
                ].concat()

}
