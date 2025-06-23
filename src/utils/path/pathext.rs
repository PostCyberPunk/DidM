use super::error::PathError;
use anyhow::{Context, Result};
use std::{fs, os::unix::fs::MetadataExt, path::Path};
//REFT: should be in util
pub trait PathExtension: AsRef<Path> {
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
                if metadata.permissions().readonly()
                    || (std::env::var("USER").unwrap() != "root" && metadata.uid() != 1000)
                {
                    return Err(PathError::NoPermission(self.to_string()).into());
                }
            }
            Err(_) => {
                return Err(PathError::NoPermission(self.to_string()).into());
            }
        }
        Ok(())
    }

    //FIX: that's not right
    fn ensure_parent_exists(&self) -> Result<&Self> {
        if !self.as_ref().exists() {
            fs::create_dir_all(self.as_ref().parent().unwrap())
                .with_context(|| PathError::CreateDirFailed(self.to_string()))?;
        }
        Ok(self)
    }
}

impl PathExtension for Path {}
