
pub trait TryFromBytes {
    fn try_from_bytes(value: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

impl TryFromBytes for String {
    fn try_from_bytes(value: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        match String::from_utf8(value) {
            Ok(s) => Ok(s),
            Err(e) => Err(Box::new(e) as Box<dyn std::error::Error>),
        }
    }
}

macro_rules! impl_try_from_bytes_for_numeric {
    ($type:ty) => {
        impl TryFromBytes for $type {
            fn try_from_bytes(value: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
                if value.len() != std::mem::size_of::<Self>() {
                    return Err("Invalid byte size".into());
                }
                let mut array = [0u8; std::mem::size_of::<Self>()];
                array.copy_from_slice(&value);
                Ok(Self::from_le_bytes(array))
            }
        }
    };
}

impl_try_from_bytes_for_numeric!(u8);
impl_try_from_bytes_for_numeric!(u16);
impl_try_from_bytes_for_numeric!(u32);
impl_try_from_bytes_for_numeric!(u64);
impl_try_from_bytes_for_numeric!(u128);
impl_try_from_bytes_for_numeric!(i8);
impl_try_from_bytes_for_numeric!(i16);
impl_try_from_bytes_for_numeric!(i32);
impl_try_from_bytes_for_numeric!(i64);
impl_try_from_bytes_for_numeric!(i128);
