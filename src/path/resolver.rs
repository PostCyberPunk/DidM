use super::error::PathError;
use anyhow::{Context, Result};
use std::{
    env,
    path::{Path, PathBuf},
};

pub struct PathResolver {
    pub check_env: bool,
}
impl PathResolver {
    pub fn new(check_env: bool) -> Self {
        PathResolver { check_env }
    }
    // -----------Internal------
    fn expand_env_vars(&self, path: String) -> Result<String> {
        if !path.contains("$") {
            return Ok(path);
        }
        let mut expand = path;
        for (key, value) in env::vars() {
            let placeholder = format!("${}", key);
            expand = expand.replace(&placeholder, &value);
        }
        if expand.contains("$") {
            return Err(PathError::EnvVarMissing(expand).into());
        }
        Ok(expand)
    }
    fn expand_tilde(&self, path: String) -> Result<String> {
        if path.starts_with("~") {
            let home = env::var("HOME").map_err(|_| {
                PathError::EnvVarMissing("Failed to resolve `~` from $HOME ".to_string())
            })?;
            let result = path.replacen("~", &home, 1);
            return Ok(result);
        }
        Ok(path)
    }
    // -----------Public ----------------
    pub fn resolve(&self, path: &str) -> Result<PathBuf> {
        let mut resolve = path.to_string();
        resolve = self
            .expand_tilde(resolve)
            .and_then(|p| self.expand_env_vars(p))
            .with_context(|| PathError::ResolveFailed(path.to_string()))?;

        Ok(PathBuf::from(resolve))
    }
    pub fn resolve_from(&self, base_path: &Path, path: &str) -> Result<PathBuf> {
        let resolved = self.resolve(path)?;
        match resolved.is_absolute() {
            true => Ok(resolved),
            false => Ok(base_path.join(resolved)),
        }
    }
    pub fn resolve_from_or_base(&self, base_path: &Path, path: &Option<String>) -> Result<PathBuf> {
        match path {
            Some(p) => self.resolve_from(base_path, p.as_str()),
            None => Ok(base_path.to_path_buf()),
        }
    }
}
