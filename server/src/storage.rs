//! Storage abstractions for the server.
//!
//! This module defines the `Storage` trait and a thread-safe `HashMap` implementation.

use shared::consistent_hash_ring::ConsistentHashRing;
use std::collections::BTreeMap;
use std::ops::Bound::Included;
use std::sync::{Arc, Mutex};

/// A thread-safe, shared storage type.
pub type GlobalStorage = Arc<Mutex<BTreeStorage>>;

/// A trait for basic storage operations.
pub trait Storage {
    type ValueType;

    fn new() -> Self;
    /// Adds a value to the storage. Returns `true` if it was successfully added.
    fn add(&mut self, value: Self::ValueType) -> bool;
    /// Checks if a value exists in the storage.
    fn check(&self, value: &Self::ValueType) -> bool;
    /// Gets the number of elements in the storage.
    fn get_count(&self) -> usize;

    fn remove(&mut self, value: u64) -> bool;
    
    fn get_values(&self, start: u64, end: u64) -> Vec<&Self::ValueType>;

    fn get_keys_in_range(&self, start: u64, end: u64) -> Vec<u64>;
}

pub struct BTreeStorage {
    data: BTreeMap<u64, String>,
}

impl Storage for BTreeStorage {
    type ValueType = String;

    fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

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
