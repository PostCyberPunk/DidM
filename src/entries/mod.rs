mod manager;
pub use manager::EntriesManager;

mod entry;
pub use entry::*;

mod collector;
pub use collector::EntryCollector;

mod walk;
use walk::DirWalker;

mod error;
