mod apply;
mod init;

mod list;
use list::EntriesList;

pub struct EntriesManager {
    // backuper:Backuper,
    entry_list: EntriesList,
    pub is_dryrun: bool,
}
