mod apply;
mod init;
mod model;
pub use model::*;

use crate::log::Logger;

pub struct EntriesManager<'a> {
    // backuper:Backuper,
    copy_list: Vec<Entry>,
    link_list: Vec<Entry>,
    logger: &'a Logger,
    is_dryrun: bool,
}
