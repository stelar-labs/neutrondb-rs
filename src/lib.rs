
use std::error::Error;

mod bloom_filter;
mod compaction;
mod initialization;
mod flush;
pub mod store;

pub fn store(name: &str) -> Result<store::Store, Box<dyn Error>> {
    
    initialization::store(&name)?;

    let cache = initialization::cache(&name);

    let grave = initialization::grave(&name);

    let tables = initialization::tables(&name);

    let store = store::Store {
        name: String::from(name),
        cache: cache,
        grave: grave,
        tables: tables
    };

    return Ok(store)
    
}
