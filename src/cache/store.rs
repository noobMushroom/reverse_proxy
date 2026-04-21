use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use crate::cache::entry::{CacheEntry, CacheKey};
use dashmap::DashMap;
use tracing::info;

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
        let entry = self.store.get(key)?.value().clone();

        if entry.expires_at > Instant::now() {
            Some(entry)
        } else {
            info!(event = "Removing Cache", path = %key.path);

            self.store.remove(key);
            self.decrease_store_size(entry.response.body_len);

            info!(event = "Cache Removed", path = %key.path, store_size = %self.size.load(Ordering::Relaxed));
            None
        }
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
            info!(event = "Cache Added", path = %key.path, store_size = %self.size.load(Ordering::Relaxed));
            self.store.insert(key, value);
        }
    }
}
