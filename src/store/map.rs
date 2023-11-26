use crate::Store;
use std::error::Error;


impl<'a,K,V> Store<K,V> {
    pub fn map<T,F>(&self, _f: F) -> Result<Vec<T>, Box<dyn Error>>
    where F: Fn(K,V) -> T {
        Ok(vec![])
    }
}
