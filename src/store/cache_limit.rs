use crate::Store;

impl <K,V> Store<K,V> {
    pub fn cache_limit(mut self, cache_limit: u64) {
        if cache_limit >= 1_000_000 {
            self.cache_limit = cache_limit
        }
    }
}