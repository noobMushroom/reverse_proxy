use crate::cache::entry::{CacheEntry, CacheKey};
use dashmap::DashMap;

pub struct Store {
    pub store: DashMap<CacheKey, CacheEntry>,
    pub size: usize
}

impl Store {
    pub fn new() -> Self {
        Self {
            store: DashMap::new(),
            size: 0,
        }
    }
}
