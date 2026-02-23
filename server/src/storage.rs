//! Storage abstractions for the server.
//!
//! This module defines the `Storage` trait and a thread-safe `HashSetStorage` implementation.

use std::collections::HashSet;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

/// A thread-safe, shared storage type.
pub type GlobalStorage<T> = Arc<Mutex<HashSetStorage<T>>>;


/// A trait for basic storage operations.
pub trait Storage<T> {
    /// Adds a value to the storage. Returns `true` if it was successfully added.
    fn add(&mut self, value: T) -> bool;
    /// Checks if a value exists in the storage.
    fn check(&self, value: &T) -> bool;
    /// Gets the number of elements in the storage.
    fn get_count(&self) -> usize;
}

/// A storage implementation based on a `HashSet`.
pub struct HashSetStorage<T> {
    data: HashSet<T>
}

impl<T> HashSetStorage<T> {
    /// Creates a new, empty `HashSetStorage`.
    pub fn new() -> Self {
        Self { data: HashSet::new() }
    }
}

impl<T: Eq + Hash> Storage<T> for HashSetStorage<T> {
    fn add(&mut self, value: T) -> bool {
        self.data.insert(value)
    }

    fn check(&self, value: &T) -> bool {
        self.data.contains(&value)
    }
    
    fn get_count(&self) -> usize {
        self.data.len()
    }
}

