use crate::Store;
use std::error::Error;


impl<'a,K,V> Store<K,V> {
    pub fn map<T,F>(&self, _f: F) -> Result<Vec<Option<T>>, Box<dyn Error>>
    where F: Fn(K,V) -> Option<T> {
        Ok(vec![])
    }
}
