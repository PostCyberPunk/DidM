use super::{super::Entry, EntriesManager, list::EntriesList};
use crate::model::sketch::Mode;

impl EntriesManager {
    pub fn new(is_dryrun: bool) -> Self {
        Self {
            is_dryrun,
            entry_list: EntriesList::default(),
        }
    }

    pub fn add_entry(&mut self, mode: Mode, entry: Entry) {
        self.entry_list.add_entry(mode, entry);
    }
}
