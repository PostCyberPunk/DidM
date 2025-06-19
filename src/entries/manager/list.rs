use super::Entry;
use crate::model::sketch::Mode;
#[derive(Debug, Clone, Default)]

//TODO:Use trait to have two type of list
//but we have to have a static logger for that
pub struct EntriesList {
    pub copy_list: Vec<Entry>,
    pub link_list: Vec<Entry>,
}
impl EntriesList {
    pub fn add_entry(&mut self, mode: Mode, entry: Entry) {
        match mode {
            Mode::Symlink => self.link_list.push(entry),
            Mode::Copy => self.copy_list.push(entry),
        }
    }
}
