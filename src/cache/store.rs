use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use crate::cache::entry::{CacheEntry, CacheKey};
use dashmap::DashMap;

pub struct CacheStore {
    pub store: DashMap<CacheKey, CacheEntry>,
    pub size: AtomicUsize,
}

impl CacheStore {
    pub fn new() -> Self {
        Self {
            store: DashMap::new(),
            size: AtomicUsize::new(0),
        }
    }

    pub fn get(&self, key: &CacheKey) -> Option<CacheEntry> {
        if let Some(entry) = self.store.get(key) {
            let entry = entry.value();
            if entry.expires_at < Instant::now() {
                return Some(entry.clone());
            } else {
                self.store.remove(key);
                self.decrease_store_size(entry.response.body_len);
            }
        }

        None
    }

    fn decrease_store_size(&self, val: usize) {
        self.size.fetch_sub(val, Ordering::Relaxed);
    }

    fn increase_store_size(&self, val: usize) {
        self.size.fetch_add(val, Ordering::Relaxed);
    }

    pub fn insert(&self, key: CacheKey, value: CacheEntry, cache_size: usize) {
        if self.size.load(Ordering::Relaxed) + value.response.body_len < cache_size {
            self.increase_store_size(value.response.body_len);
            self.store.insert(key, value);
        }
    }
}
