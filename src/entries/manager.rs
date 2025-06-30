use crate::entries::{
    Entry,
    apply_strategy::{ActionCopy, ActionLink},
    list::EntryList,
};

use super::tree::{EntryKind, TreeManager};

pub struct EntriesManager {
    pub copy_list: EntryList<ActionCopy>,
    pub link_list: EntryList<ActionLink>,
    pub skip_list: Vec<Entry>,
    pub error_list: Vec<(Entry, String)>,
    pub is_dryrun: bool,
}

impl EntriesManager {
    pub fn new(is_dryrun: bool) -> Self {
        Self {
            copy_list: EntryList::new(),
            link_list: EntryList::new(),
            skip_list: Vec::new(),
            error_list: Vec::new(),
            is_dryrun,
        }
    }

    // pub fn add_copies(&mut self, entries: Vec<Entry>) {
    //     self.copy_list.add_entries(entries);
    // }
    pub fn add_copy(&mut self, entry: Entry) {
        self.copy_list.add_entry(entry);
    }

    // pub fn add_links(&mut self, entries: Vec<Entry>) {
    //     self.link_list.add_entries(entries);
    // }
    pub fn add_link(&mut self, entry: Entry) {
        self.link_list.add_entry(entry);
    }
    pub fn skip_entry(&mut self, entry: Entry) {
        self.skip_list.push(entry);
    }
    pub fn add_error(&mut self, error_entry: (Entry, String)) {
        self.error_list.push(error_entry);
    }

    pub fn apply_all(&self) {
        let _ = self.link_list.apply_entries(self.is_dryrun);
        let _ = self.copy_list.apply_entries(self.is_dryrun);
    }

    pub fn fill_tree(&self, tree: &mut TreeManager) {
        for entry in &self.copy_list.entries {
            tree.insert_path(&entry.target_path, EntryKind::Copy);
        }
        for entry in &self.link_list.entries {
            tree.insert_path(&entry.target_path, EntryKind::Link);
        }
        for entry in &self.skip_list {
            tree.insert_path(&entry.target_path, EntryKind::Skip);
        }
        // for (entry, error) in &self.error_list {
        //     tree.insert_path(&entry.target_path, EntryKind::Error(error.clone()));
        // }
    }
}
