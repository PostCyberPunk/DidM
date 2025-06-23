use crate::entries::{
    Entry,
    apply_strategy::{ActionCopy, ActionLink},
    list::EntryList,
};

pub struct EntriesManager {
    copy_list: EntryList<ActionCopy>,
    link_list: EntryList<ActionLink>,
    pub is_dryrun: bool,
}

impl EntriesManager {
    pub fn new(is_dryrun: bool) -> Self {
        Self {
            copy_list: EntryList::new(),
            link_list: EntryList::new(),
            is_dryrun,
        }
    }

    pub fn add_copies(&mut self, entries: Vec<Entry>) {
        self.copy_list.add_entries(entries);
    }
    pub fn add_copy(&mut self, entry: Entry) {
        self.copy_list.add_entry(entry);
    }

    pub fn add_links(&mut self, entries: Vec<Entry>) {
        self.link_list.add_entries(entries);
    }
    pub fn add_link(&mut self, entry: Entry) {
        self.link_list.add_entry(entry);
    }

    pub fn apply_all(&self) {
        let _ = self.link_list.apply_entries(self.is_dryrun);
        let _ = self.copy_list.apply_entries(self.is_dryrun);
    }
}
