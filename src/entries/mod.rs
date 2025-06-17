mod entry;
mod init;

use crate::{helpers::Helpers, log::Logger};
pub use entry::Entry;

pub struct AllEntries<'a> {
    // backuper:Backuper,
    copy_list: Vec<Entry>,
    link_list: Vec<Entry>,
    helpers: &'a Helpers,
    logger: &'a Logger,
    is_dryrun: bool,
}
