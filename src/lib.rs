pub mod crypto;
pub mod onion;
pub mod worker;

pub use crypto::*;
pub use onion::*;
pub use worker::*;

use std::sync::atomic::{AtomicU64, Ordering};

/// Global counters for tracking generation statistics
pub static GENERATED_COUNT: AtomicU64 = AtomicU64::new(0);
pub static FOUND_COUNT: AtomicU64 = AtomicU64::new(0);

/// Result structure for generated onion addresses
#[derive(Debug, Clone)]
pub struct OnionResult {
    pub hostname: String,
    pub public_key: String,
    pub private_key: String,
}

/// Configuration for the onion generator
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    pub prefixes: Vec<String>,
    pub num_workers: usize,
    pub update_interval: u64,
}

impl GeneratorConfig {
    pub fn new(prefixes: Vec<String>) -> Self {
        let num_workers = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        
        Self {
            prefixes,
            num_workers,
            update_interval: 30,
        }
    }

    pub fn with_workers(mut self, num_workers: usize) -> Self {
        self.num_workers = num_workers;
        self
    }

    pub fn with_update_interval(mut self, interval: u64) -> Self {
        self.update_interval = interval;
        self
    }
}

/// Get current generation statistics
pub fn get_stats() -> (u64, u64) {
    (
        GENERATED_COUNT.load(Ordering::Relaxed),
        FOUND_COUNT.load(Ordering::Relaxed),
    )
}

/// Increment the generated counter
pub fn increment_generated() {
    GENERATED_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Increment the found counter
pub fn increment_found() {
    FOUND_COUNT.fetch_add(1, Ordering::Relaxed);
}
