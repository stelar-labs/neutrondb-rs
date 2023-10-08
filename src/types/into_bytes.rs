pub trait IntoBytes {
    fn into_bytes(&self) -> Vec<u8>;
}

macro_rules! impl_into_bytes_from_le_bytes {
    ($type:ty) => {
        impl IntoBytes for $type {
            fn into_bytes(&self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }
        }
        
        impl<'a> IntoBytes for &$type {
            fn into_bytes(&self) -> Vec<u8> {
                (*self).to_le_bytes().to_vec()
            }
        }
    };
}

impl_into_bytes_from_le_bytes!(u8);
impl_into_bytes_from_le_bytes!(u16);
impl_into_bytes_from_le_bytes!(u32);
impl_into_bytes_from_le_bytes!(u64);
impl_into_bytes_from_le_bytes!(u128);
impl_into_bytes_from_le_bytes!(i8);
impl_into_bytes_from_le_bytes!(i16);
impl_into_bytes_from_le_bytes!(i32);
impl_into_bytes_from_le_bytes!(i64);
impl_into_bytes_from_le_bytes!(i128);

macro_rules! impl_into_bytes_from_as_bytes {
    ($type:ty) => {
        impl IntoBytes for $type {
            fn into_bytes(&self) -> Vec<u8> {
                self.as_bytes().to_vec()
            }
        }
        
        impl<'a> IntoBytes for &$type {
            fn into_bytes(&self) -> Vec<u8> {
                (*self).as_bytes().to_vec()
            }
        }
    };
}

impl_into_bytes_from_as_bytes!(String);
impl_into_bytes_from_as_bytes!(str);

macro_rules! impl_into_bytes_from_to_vec {
    ($type:ty) => {
        impl IntoBytes for $type {
            fn into_bytes(&self) -> Vec<u8> {
                self.to_vec()
            }
        }
        
        impl<'a> IntoBytes for &$type {
            fn into_bytes(&self) -> Vec<u8> {
                (*self).to_vec()
            }
        }
    };
}

impl_into_bytes_from_to_vec!([u8]);
