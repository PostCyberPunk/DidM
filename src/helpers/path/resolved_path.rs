use super::PathError;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ResolvedPath {
    //TODO: this should be a path instead of pathbuf?
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

    //------------------------
    pub fn to_parent(&self) -> Result<ResolvedPath> {
        if self.path == PathBuf::from("/") {
            return Err(PathError::NoParent.into());
        }
        let path = self.path.parent().unwrap().to_path_buf();
        //FIX: raw_path is not safe after all
        // let raw_path = PathBuf::from(&self.raw)
        //     .parent()
        //     .unwrap()
        //     .display()
        //     .to_string();
        // let raw: String = match raw_path.as_str() {
        //     "." | "./" => String::from(".."),
        //     "" => path.display().to_string(),
        //     _ => raw_path,
        // };
        //TODO: i am lazy ,so lets use absolute...
        let raw = path.display().to_string();
        Ok(ResolvedPath { path, raw })
    }
    pub fn into_parent(self) -> Result<Self> {
        self.to_parent()
    }
}
impl PartialEq for ResolvedPath {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}
