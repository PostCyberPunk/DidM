use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

//TODO: too many redundant resolve, maybe we should have RawPathHandler and PathHandler,get the
//normal one from raw
#[derive(Debug, Error)]
pub enum PathError {
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Missing environment variable in {0}")]
    EnvVarMissing(String),

    #[error("Failed to create directory: {0}")]
    CreateDirFailed(String),

    #[error("File {0} already existed in {1}")]
    FileExists(String, String),
}

pub struct RawPathHandler {
    raw_path: String,
}
impl RawPathHandler {
    pub fn new(path: Option<&str>) -> Self {
        Self {
            raw_path: match path {
                Some(p) => p.to_string(),
                None => ".".to_string(),
                // None => std::env::current_dir(),
            },
        }
    }
    pub fn get_raw_path(&self) -> &str {
        &self.raw_path
    }
    pub fn resolve(self) -> Result<PathHandler> {
        let handler = self.expand_tilde().expand_env_vars()?;
        let path_buf = PathBuf::from(&handler.raw_path);

        Ok(PathHandler {
            raw_path: handler.raw_path,
            path_buf,
        })
    }
    fn expand_env_vars(mut self) -> Result<Self> {
        let mut expanded = self.raw_path.to_string();
        for (key, value) in env::vars() {
            let placeholder = format!("${}", key);
            expanded = expanded.replace(&placeholder, &value);
        }
        if expanded.contains("$") {
            return Err(PathError::EnvVarMissing(expanded).into());
        }
        self.raw_path = expanded;
        Ok(self)
    }
    fn expand_tilde(mut self) -> Self {
        let path = &self.raw_path;
        if path.starts_with("~") {
            self.raw_path = path.replacen("~", "$home", 1);
        }
        self
    }
}
pub struct PathHandler {
    raw_path: String,
    path_buf: PathBuf,
}
impl PathHandler {
    pub fn get_absolute_path(&self) -> Result<PathBuf> {
        Ok(self.path_buf.canonicalize()?)
    }

    pub fn exists(&self) -> Result<bool> {
        Ok(self.path_buf.exists())
    }

    pub fn is_file(&self) -> Result<bool> {
        Ok(self.path_buf.is_file())
    }

    pub fn is_directory(&self) -> Result<bool> {
        Ok(self.path_buf.is_dir())
    }

    pub fn ensure_path_exists(&self) -> Result<()> {
        let path_buf = &self.path_buf;
        if !path_buf.exists() {
            fs::create_dir_all(&path_buf).with_context(|| {
                PathError::CreateDirFailed(path_buf.to_string_lossy().to_string())
            })?;
        }
        Ok(())
    }

    pub fn find_file(&self, filename: &str) -> Result<PathBuf> {
        let file_path = self.path_buf.join(filename);
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
        let file_path = self.path_buf.join(filename);
        if file_path.exists() {
            Err(PathError::FileExists(filename.to_string(), self.raw_path.to_string()).into())
        } else {
            Ok(file_path)
        }
    }
}
