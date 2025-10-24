// Make netabase crates available via absolute path for macro hygiene
#[cfg(feature = "netabase")]
extern crate netabase_store;
#[cfg(feature = "netabase")]
extern crate netabase_deps;

pub mod fetchers;

#[cfg(not(target_arch = "wasm32"))]
pub mod storage;

#[cfg(not(target_arch = "wasm32"))]
pub mod utils;

// New in-memory processing modules
pub mod data_processor;
pub mod fetcher;

// Masterlist manager
pub mod masterlist_manager;

// Re-export the main API
pub use fetcher::{GdeltFetcher, StorageMode};
pub use masterlist_manager::MasterlistManager;
