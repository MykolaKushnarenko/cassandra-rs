//! Storage abstractions for the server.
//!
//! This module defines the `Storage` trait and a thread-safe `HashMap` implementation.

use shared::consistent_hash_ring::ConsistentHashRing;
use std::collections::BTreeMap;
use std::ops::Bound::Included;
use std::sync::{Arc, Mutex};

/// A thread-safe, shared storage type.
pub type GlobalStorage<T> = Arc<Mutex<BTreeStorage<T>>>;

/// A trait for basic storage operations.
pub trait Storage<T> {
    /// Adds a value to the storage. Returns `true` if it was successfully added.
    fn add(&mut self, value: T) -> bool;
    /// Checks if a value exists in the storage.
    fn check(&self, value: &T) -> bool;
    /// Gets the number of elements in the storage.
    fn get_count(&self) -> usize;

    fn remove(&mut self, value: u64) -> bool;

    fn get_value_by_key(&self, key: u64) -> Option<&T>;

    fn get_values(&self, start: u64, end: u64) -> Vec<&T>;

    fn get_keys_in_range(&self, start: u64, end: u64) -> Vec<u64>;
}

pub struct BTreeStorage<T> {
    data: BTreeMap<u64, T>,
}

impl BTreeStorage<String> {
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }
}

impl Storage<String> for BTreeStorage<String> {
    fn add(&mut self, value: String) -> bool {
        let hash = ConsistentHashRing::calculate_hash(value.as_str());
        let result = self.data.insert(hash, value);
        result.is_none()
    }

    fn check(&self, value: &String) -> bool {
        let hash = ConsistentHashRing::calculate_hash(value.as_str());
        self.data.contains_key(&hash)
    }

    fn get_count(&self) -> usize {
        self.data.len()
    }

    fn remove(&mut self, value: u64) -> bool {
        self.data.remove(&value).is_some()
    }

    fn get_value_by_key(&self, key: u64) -> Option<&String> {
        self.data.get(&key)
    }

    fn get_values(&self, start: u64, end: u64) -> Vec<&String> {
        self.data
            .range((Included(start), Included(end)))
            .map(|(_, v)| v)
            .collect()
    }

    fn get_keys_in_range(&self, start: u64, end: u64) -> Vec<u64> {
        self.data
            .range((Included(start), Included(end)))
            .map(|(k, _)| *k)
            .collect()
    }
}
