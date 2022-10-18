use std::convert::TryInto;
use std::error::Error;
use astro_format;

pub fn run<K,V>(bytes: &[u8]) -> Result<Vec<(K, V)>, Box<dyn Error>>

    where
    
        <K as TryFrom<Vec<u8>>>::Error: std::error::Error,
        
        K: TryFrom<Vec<u8>> + Clone,
        
        V: TryFrom<Vec<u8>>
        
            {
                
                let bloom_filter_length_buffer = &bytes[0..9];

                let bloom_size = usize::from_le_bytes(bloom_filter_length_buffer.try_into()?);

                let keys_index = 8 + bloom_size;
                
                let keys_length = &bytes[keys_index..(keys_index + 8)];

                let keys_size = usize::from_le_bytes(keys_length.try_into()?);

                let keys_buffer_index = keys_index + 8;

                let keys_buffer = &bytes[keys_buffer_index..(keys_buffer_index + keys_size)];

                let key_index_encoded = astro_format::decode(keys_buffer)?;

                let mut key_index = Vec::new();

                for x in key_index_encoded.iter() {

                    let key_index_decoded = astro_format::decode(x)?;

                    if key_index_decoded.len() == 2 {

                        match K::try_from(key_index_decoded[0].to_vec()) {

                            Ok(k) => {

                                let index = u64::from_be_bytes(key_index_decoded[1].try_into().unwrap());

                                key_index.push((k, index))

                            },

                            _ => ()
                            
                        }

                    }

                }

                let mut key_values = Vec::new();

                for i in 0..key_index.len() {

                    let start = key_index[i].1 as usize;

                    let end = if i == key_index.len() - 1 {
                        bytes.len()
                    } else {
                        key_index[i + 1].1 as usize
                    };

                    let value_size = end - start;

                    let value_index = keys_index + keys_size + start;

                    let value_bytes = bytes[value_index..(value_index + value_size)].to_vec();

                    match V::try_from(value_bytes) {

                        Ok(v) => { key_values.push((key_index[i].0.clone(), v)); },

                        _ => ()
                        
                    }
                    
                }

                Ok(key_values)

}