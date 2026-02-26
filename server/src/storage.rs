//! Storage abstractions for the server.
//!
//! This module defines the `Storage` trait and a thread-safe `HashMap` implementation.

use shared::consistent_hash::ConsistentHash;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A thread-safe, shared storage type.
pub type GlobalStorage<T> = Arc<Mutex<HashMapStorage<T>>>;

/// A trait for basic storage operations.
pub trait Storage<T> {
    /// Adds a value to the storage. Returns `true` if it was successfully added.
    fn add(&mut self, value: T) -> bool;
    /// Checks if a value exists in the storage.
    fn check(&self, value: &T) -> bool;
    /// Gets the number of elements in the storage.
    fn get_count(&self) -> usize;
}

pub struct HashMapStorage<T> {
    data: HashMap<u64, T>
}

impl HashMapStorage<String> {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }
}

impl Storage<String> for HashMapStorage<String>  {
    fn add(&mut self, value: String) -> bool {
        let hash = ConsistentHash::calculate_hash(value.as_str());
        let result = self.data.insert(hash, value);
        result.is_none()
    }

    fn check(&self, value: &String) -> bool {
        let hash = ConsistentHash::calculate_hash(value.as_str());
        self.data.contains_key(&hash)
    }

    fn get_count(&self) -> usize {
        self.data.len()
    }
}