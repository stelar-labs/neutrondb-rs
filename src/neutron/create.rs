use astro_format::arrays;
use std::collections::BTreeMap;

pub fn create(bloom: Vec<u8>, key_values: BTreeMap<String, String>) -> Vec<u8> {

    let mut keys = Vec::new();

    let mut indices = Vec::new();

    let mut values = Vec::new();

    let mut index = 0_u64;

    for (key, value) in key_values.iter() {

        let k_bytes = key.as_bytes();

        keys.push(k_bytes);

        let v_bytes = value.as_bytes();

        values.push(v_bytes);

        let i_bytes = index.to_le_bytes();

        indices.push(i_bytes);

        index += value.len() as u64;
        
    }

    let indices_slices = indices
        .iter()
        .map(|x| &x[..])
        .collect::<Vec<&[u8]>>();

    let keys_buffer = arrays::encode(&keys);

    let index_buffer = arrays::encode(&indices_slices);

    let value_buffer = values.concat();

    [
        (bloom.len() as u64).to_le_bytes().to_vec(), bloom,
        (keys_buffer.len() as u64).to_le_bytes().to_vec(), keys_buffer,
        (index_buffer.len() as u64).to_le_bytes().to_vec(), index_buffer,
        value_buffer
    ].concat()

}