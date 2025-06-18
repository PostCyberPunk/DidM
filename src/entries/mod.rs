mod apply;
mod init;

mod entry;
pub use entry::Entry;

mod walk;
use walk::WalkerContext;

mod error;

use crate::{helpers::Helpers, log::Logger};

//TODO: we need a better name for this
pub struct AllEntries<'a> {
    // backuper:Backuper,
    copy_list: Vec<Entry>,
    link_list: Vec<Entry>,
    helpers: &'a Helpers,
    logger: &'a Logger,
    is_dryrun: bool,
}
