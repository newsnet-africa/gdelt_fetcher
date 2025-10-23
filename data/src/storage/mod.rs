// Netabase models for masterfilelist
#[cfg(feature = "netabase")]
pub mod masterfilelist_models;

// Netabase-powered store (works on both native and WASM)
#[cfg(feature = "netabase")]
pub mod netabase_masterfilelist_store;

// Fallback: Simple cross-platform store (works without netabase)
#[cfg(not(feature = "netabase"))]
pub mod simple_masterfilelist_store;

// Re-export the appropriate store based on features
#[cfg(feature = "netabase")]
pub use netabase_masterfilelist_store::NetabaseMasterfilelistStore as MasterfilelistStore;

#[cfg(not(feature = "netabase"))]
pub use simple_masterfilelist_store::SimpleMasterfilelistStore as MasterfilelistStore;
