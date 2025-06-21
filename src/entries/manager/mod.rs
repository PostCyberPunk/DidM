mod apply;
mod init;

mod list;
use list::EntriesList;

use crate::log::Logger;

pub struct EntriesManager<'a> {
    // backuper:Backuper,
    entry_list: EntriesList,
    pub logger: &'a Logger,
    pub is_dryrun: bool,
}
