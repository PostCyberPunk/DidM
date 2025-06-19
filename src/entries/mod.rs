mod manager;
pub use manager::EntriesManager;

mod bakcup;
use bakcup::BackupState;

mod walk;
use walk::DirWalker;

mod error;
