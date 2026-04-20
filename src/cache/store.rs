use crate::cache::entry::{CacheEntry, CacheKey};
use dashmap::DashMap;

pub struct CacheStore {
    pub store: DashMap<CacheKey, CacheEntry>,
    pub size: usize
}

impl CacheStore {
    pub fn new() -> Self {
        Self {
            store: DashMap::new(),
            size: 0,
        }
    }


    // pub fn get(&self, key: &CacheKey) -> Option<CacheEntry> {
    //     if let Some(entry) = self.store.get(key)     {
    //         let entry = entry.clone()
    //     }
    // }
}
