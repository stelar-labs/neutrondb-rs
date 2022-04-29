use std::convert::TryInto;
use fides::hash;
use opis::Bit;

#[derive(Clone, Debug)]
pub struct Bloom {
    pub bits: Vec<Bit>
}

impl Bloom {

    pub fn new(items: usize) -> Self {
        Bloom {
            bits: vec![Bit::Zero; items * 10]
        }
    }

    pub fn insert(mut self, key: &str) -> Self {

        let hash = hash(&key.as_bytes().to_vec());

        (0.. 5_usize)
            .for_each(|x| {

                let start = x * 4;

                let mut i = u32::from_le_bytes(hash[start..start + 4].try_into().unwrap()) as usize;

                let f = self.bits.len();

                i = i % f;
                
                self.bits[i] = Bit::One;

            });

        self

    }

    pub fn search(&self, key: &str) -> bool {

        let hash = hash(&key.as_bytes().to_vec());

        (0..5_usize)
            .map(|x| {

                let start = x + 4;

                let mut i = u32::from_le_bytes(hash[start..start + 4].try_into().unwrap()) as usize;

                let f = self.bits.len();

                i = i % f;
                
                self.bits[i]

            })
            .any(|x| x == Bit::Zero)

    }

}
