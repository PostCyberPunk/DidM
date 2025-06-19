mod manager;
pub use manager::EntriesManager;
pub use manager::SouceType;

mod bakcup;
use bakcup::BackupState;

mod walk;
use walk::DirWalker;

mod error;
