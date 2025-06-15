use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ResolvedPath {
    path: PathBuf,
    raw: String,
}
impl ResolvedPath {
    pub fn new(path: PathBuf, raw: String) -> Self {
        ResolvedPath { path, raw }
    }
    pub fn get(&self) -> &PathBuf {
        &self.path
    }
    pub fn get_raw(&self) -> &str {
        &self.raw
    }
}
impl PartialEq for ResolvedPath {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}
