use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PathError {
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Environment variable `{0}` is missing")]
    EnvVarMissing(String),

    #[error("Failed to create directory: {0}")]
    CreateDirFailed(String),

    #[error("File {0} already existed in {1}")]
    FileExists(String, String),
}

pub struct PathHandler {
    raw_path: String,
}

impl PathHandler {
    pub fn new(path: &str) -> Self {
        Self {
            raw_path: path.to_string(),
        }
    }
    pub fn get_raw_path(&self) -> &str {
        &self.raw_path
    }
    pub fn resolve(&self) -> Result<PathBuf> {
        let expanded_path = self.expand_env_vars(&self.raw_path)?;
        Ok(PathBuf::from(expanded_path))
    }
    pub fn get_absolute_path(&self) -> Result<PathBuf> {
        let path_buf = self.resolve()?;
        Ok(path_buf.canonicalize()?)
    }

    pub fn exists(&self) -> Result<bool> {
        Ok(self.resolve()?.exists())
    }

    pub fn is_file(&self) -> Result<bool> {
        Ok(self.resolve()?.is_file())
    }

    pub fn is_directory(&self) -> Result<bool> {
        Ok(self.resolve()?.is_dir())
    }

    pub fn ensure_path_exists(&self) -> Result<PathBuf> {
        let path_buf = self.resolve()?;
        if !path_buf.exists() {
            fs::create_dir_all(&path_buf).with_context(|| {
                PathError::CreateDirFailed(path_buf.to_string_lossy().to_string())
            })?;
        }
        Ok(path_buf)
    }

    pub fn find_file(&self, filename: &str) -> Result<PathBuf> {
        let file_path = self.resolve()?.join(filename);
        if file_path.exists() {
            Ok(file_path)
        } else {
            Err(PathError::InvalidPath(format!(
                "File '{}' not found in {}",
                filename, self.raw_path
            ))
            .into())
        }
    }
    // TODO: fix this shit...
    // NOTE: emm...that's a pretty fucked up name,and very bad practise
    pub fn find_file_or_ok(&self, filename: &str) -> Result<PathBuf> {
        let file_path = self.resolve()?.join(filename);
        if file_path.exists() {
            Err(PathError::FileExists(filename.to_string(), self.raw_path.to_string()).into())
        } else {
            Ok(file_path)
        }
    }

    fn expand_env_vars(&self, path: &str) -> Result<String> {
        let mut expanded = path.to_string();
        for (key, value) in env::vars() {
            let placeholder = format!("${}", key);
            expanded = expanded.replace(&placeholder, &value);
        }
        if expanded.contains("$") {
            return Err(PathError::EnvVarMissing(expanded).into());
        }
        Ok(expanded)
    }
}
