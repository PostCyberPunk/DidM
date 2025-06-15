use super::error::PathError;
use anyhow::{Context, Result};
use std::{env, path::PathBuf};

pub struct PathResolver {
    pub check_env: bool,
}
impl PathResolver {
    pub fn resolve(&self, path: &str) -> Result<PathBuf> {
        let mut resolve = path.to_string();
        resolve = self
            .expand_tilde(resolve)
            .and_then(|p| self.expand_env_vars(p))
            .with_context(|| PathError::ResolveFailed)?;

        if resolve.contains("$") {
            return Err(PathError::EnvVarMissing(resolve).into());
        }
        Ok(PathBuf::from(resolve))
    }
    fn expand_env_vars(&self, path: String) -> Result<String> {
        if !path.contains("$") {
            return Ok(path);
        }
        let mut expand = path;
        for (key, value) in env::vars() {
            let placeholder = format!("${}", key);
            expand = expand.replace(&placeholder, &value);
        }
        Ok(expand)
    }
    fn expand_tilde(&self, path: String) -> Result<String> {
        if path.starts_with("~") {
            let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
            let result = path.replacen("~", &home, 1);
            return Ok(result);
        }
        Ok(path)
    }
}
