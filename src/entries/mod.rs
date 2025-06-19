mod manager;
pub use manager::EntriesManager;
pub use manager::Entry;

mod walk;
use walk::DirWalker;

mod error;
