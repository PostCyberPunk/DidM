mod error;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub use error::PathError;
//TODO: refactor this first
//TODO: We have to remember to resolve the path before using it.
//But, introduce a new struct that repsent the resolved path ,that does not feel right...

pub trait PathBufExtension: AsRef<Path> {
    fn to_string(&self) -> String {
        self.as_ref().to_string_lossy().to_string()
    }
    fn check_dir(&self) -> Result<()> {
        if !self.as_ref().is_dir() {
            return Err(PathError::NotDir(self.to_string()).into());
        }
        Ok(())
    }
    fn check_permission(&self) -> Result<()> {
        match fs::metadata(self) {
            Ok(metadata) => {
                if metadata.permissions().readonly() {
                    return Err(PathError::NoPermission(self.to_string()).into());
                }
            }
            Err(_) => {
                return Err(PathError::NoPermission(self.to_string()).into());
            }
        }
        Ok(())
    }

    fn ensure_parent_exists(&self) -> Result<&Self> {
        if !self.as_ref().exists() {
            fs::create_dir_all(self.as_ref().parent().unwrap())
                .with_context(|| PathError::CreateDirFailed(self.to_string()))?;
        }
        Ok(self)
    }
    // fn find_file(&self, filename: &str) -> Result<Self>;
}

impl PathBufExtension for PathBuf {}
// fn is_unresolved_absolute(&self) -> bool {
//     self.starts_with("$") || self.starts_with("~") || self.is_absolute()
// }
// fn resolved_from(self, base_path: &Path) -> Result<Self> {
//PERF: I decide to fart with my pants off
//but this time ,linter feels good about it
//------------------------------------
// if self.is_unresolved_absolute() {
//     return self.resolve();
// }
// let resolved = self.resolve()?;
// Ok(base_path.join(resolved))
// }
