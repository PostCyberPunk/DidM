mod apply;
mod init;

mod list;
use list::EntriesList;

use crate::log::Logger;

pub struct EntriesManager<'a> {
    // backuper:Backuper,
    entry_list: EntriesList,
    logger: &'a Logger,
    is_dryrun: bool,
}
