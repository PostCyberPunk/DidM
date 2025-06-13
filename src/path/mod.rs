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

    #[error("Failed to create parent directory: {0}")]
    CreateDirFailed(String),

    #[error("File {0} already existed in {1}")]
    FileExists(String, String),
}

//TODO: We have to remember to resolve the path before using it.
//But, introduce a new struct that repsent the resolved path ,that does not feel right...
pub trait PathBufExtension: Sized {
    fn to_str_or_null(&self) -> &str
    where
        Self: AsRef<Path>,
    {
        self.as_ref().to_str().unwrap_or("")
    }
    fn to_string(&self) -> String
    where
        Self: AsRef<Path>,
    {
        self.as_ref().to_string_lossy().to_string()
    }

    fn resolve(self) -> Result<Self>;
    fn expand_env_vars(self) -> Result<Self>;
    fn expand_tilde(self) -> Self;

    fn resolve_or_from(&self, path: &Option<String>) -> Result<PathBuf>;
    fn is_unresolved_absolute(&self) -> bool;
    fn resolved_from(self, base_path: &Path) -> Result<Self>;

    fn ensure_parent_exists(&self) -> Result<&Self>;
    fn find_file(&self, filename: &str) -> Result<Self>;
    fn find_file_or_ok(&self, filename: &str) -> Result<Self>;
}

impl PathBufExtension for PathBuf {
    fn resolve(self) -> Result<Self> {
        let resolved = self.expand_tilde().expand_env_vars()?;
        let raw_path = resolved.to_string();
        if raw_path.contains("$") {
            return Err(PathError::EnvVarMissing(raw_path).into());
        }
        Ok(resolved)
    }
    fn expand_env_vars(mut self) -> Result<Self> {
        let mut expanded = self.to_string();
        if !expanded.contains("$") {
            return Ok(self);
        }
        for (key, value) in env::vars() {
            let placeholder = format!("${}", key);
            expanded = expanded.replace(&placeholder, &value);
        }
        self = PathBuf::from(expanded);
        Ok(self)
    }
    fn expand_tilde(mut self) -> Self {
        if let Some(path) = self.to_str() {
            if path.starts_with("~") {
                let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
                self = PathBuf::from(path.replacen("~", &home, 1));
            }
        }
        self
    }

    fn is_unresolved_absolute(&self) -> bool {
        self.starts_with("$") || self.starts_with("~") || self.is_absolute()
    }
    fn resolved_from(self, base_path: &Path) -> Result<Self> {
        let resolved = self.resolve()?;
        match resolved.is_absolute() {
            true => Ok(resolved),
            false => Ok(base_path.join(resolved)),
        }
        //PERF: I decide to fart with my pants off
        //but this time ,linter feels good about it
        //------------------------------------
        // if self.is_unresolved_absolute() {
        //     return self.resolve();
        // }
        // let resolved = self.resolve()?;
        // Ok(base_path.join(resolved))
    }
    fn resolve_or_from(&self, path: &Option<String>) -> Result<PathBuf> {
        match path {
            Some(dir) => PathBuf::from(dir).resolved_from(self),
            None => Ok(self.clone()),
        }
    }

    fn ensure_parent_exists(&self) -> Result<&Self> {
        if !self.exists() {
            fs::create_dir_all(self.parent().unwrap())
                .with_context(|| PathError::CreateDirFailed(self.to_string()))?;
        }
        Ok(self)
    }

    fn find_file(&self, filename: &str) -> Result<Self> {
        let file_path = self.join(filename);
        if file_path.exists() {
            Ok(file_path)
        } else {
            Err(PathError::InvalidPath(format!(
                "File '{}' not found in {}",
                filename,
                self.to_str_or_null()
            ))
            .into())
        }
    }
    // TODO: fix this shit...
    // NOTE: emm...that's a pretty fucked up name,and very bad practise
    fn find_file_or_ok(&self, filename: &str) -> Result<Self> {
        let file_path = self.join(filename);
        if file_path.exists() {
            Err(PathError::FileExists(filename.to_string(), self.to_string()).into())
        } else {
            Ok(file_path)
        }
    }
}
