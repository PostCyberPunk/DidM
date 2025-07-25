use super::PathError;
use anyhow::Result;
use path_absolutize::Absolutize;
use std::path::{Path, PathBuf};

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
    pub fn as_path(&self) -> &Path {
        &self.path
    }
    pub fn into_pathbuf(self) -> PathBuf {
        self.path
    }
    pub fn get_raw(&self) -> &str {
        &self.raw
    }
    //------------------------
    pub fn into_parent(mut self) -> Result<Self> {
        if self.path == Path::new("/") {
            return Err(PathError::NoParent.into());
        }
        self.path.pop();
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
        // self.path
        Ok(self)
    }
    pub fn to_parent(&self) -> Result<ResolvedPath> {
        self.clone().into_parent()
    }
    pub fn to_child(&self, filename: &str, should_check_exist: bool) -> Result<ResolvedPath> {
        self.clone().into_child(filename, should_check_exist)
    }

    pub fn into_child(mut self, filename: &str, should_check_exist: bool) -> Result<Self> {
        self.path.push(filename);
        self.path = self.path.absolutize()?.into();
        if should_check_exist && !self.path.exists() {
            return Err(PathError::NotExists(self.path).into());
        };
        // let raw = PathBuf::from(self.raw).join(filename).display().to_string();
        self.raw.push('/');
        self.raw.push_str(filename);
        Ok(self)
    }
}
impl PartialEq for ResolvedPath {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}
