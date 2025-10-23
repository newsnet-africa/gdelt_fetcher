pub mod event_table;
pub mod gkg_table;
pub mod lookup_types;
pub mod mention_table;
pub mod masterfilelist; // Old implementation
pub mod masterlist;     // New netabase-powered implementation
pub mod geg_table;

pub trait DatabaseTable {}
