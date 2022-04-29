use astro_format::arrays;
use std::collections::BTreeMap;

pub fn create(bloom: Vec<u8>, key_values: BTreeMap<String, String>) -> Vec<u8> {

    let mut index: Vec<(Vec<u8>, u64)> = Vec::new();

    let mut values: Vec<u8> = Vec::new();

    for (key, value) in key_values {
        
        let key_bytes = key.into_bytes();

        index.push((key_bytes, values.len() as u64));
        
        let value_bytes = value.into_bytes();

        values = [values, value_bytes].concat()
        
    }

    let index_buffer = arrays::encode(&
        index
            .into_iter()
            .map(|x| {
                
                arrays::encode(&vec![x.0.clone(), x.1.to_be_bytes().to_vec()])

            })
            .collect()
    );

    let index_len = (index_buffer.len() as u64).to_be_bytes().to_vec();

    let bloom_len = (bloom.len() as u64).to_be_bytes().to_vec();

    [index_len, bloom_len, index_buffer, bloom, values].concat()

}