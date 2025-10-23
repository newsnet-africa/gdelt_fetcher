pub mod fetchers;

#[cfg(not(target_arch = "wasm32"))]
pub mod storage;

#[cfg(not(target_arch = "wasm32"))]
pub mod utils;

// New in-memory processing modules
pub mod data_processor;
pub mod fetcher;

// Masterlist manager - now uses new netabase_store API
#[cfg(not(feature = "netabase"))]
pub mod masterlist_manager;

// Re-export the main API
pub use fetcher::{GdeltFetcher, StorageMode};
#[cfg(not(feature = "netabase"))]
pub use masterlist_manager::MasterlistManager;
