use super::ResolvedPath;
use crate::{cli::prompt::confirm, helpers::path::PathError};
use anyhow::{Context, Result};
use std::{env, path::PathBuf};

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
    fn check_symlink_then_absolute(&self, path: &str) -> Result<PathBuf> {
        let pathbuf = PathBuf::from(&path);
        if pathbuf.is_symlink()
            && !confirm(&format!(
                "{} is a symlink,this may lead to some unexcepted issue,do you want to continue?",
                path
            ))
        {
            return Err(PathError::UnresolvedSymlink("User cancelled".to_string()).into());
        }
        pathbuf
            .canonicalize()
            .map_err(|e| PathError::Unknown(e.to_string()).into())
    }
    // -----------Public ----------------
    pub fn resolve(&self, path: &str) -> Result<ResolvedPath> {
        let resolve = self
            .expand_tilde(path.to_string())
            .and_then(|p| self.expand_env_vars(p))
            .and_then(|p| self.check_symlink_then_absolute(&p))
            .with_context(|| PathError::ResolveFailed(path.to_string()))?;

        Ok(ResolvedPath::new(resolve, path.to_string()))
    }
    pub fn resolve_from(&self, base_path: &ResolvedPath, path: &str) -> Result<ResolvedPath> {
        let resolved = self.resolve(path)?;
        if resolved.get().is_absolute() {
            Ok(resolved)
        } else {
            Ok(ResolvedPath::new(
                base_path.get().join(resolved.get()),
                path.to_string(),
            ))
        }
    }
    pub fn resolve_from_or_base(
        &self,
        base_path: &ResolvedPath,
        path: &Option<String>,
    ) -> Result<ResolvedPath> {
        match path {
            Some(p) => self.resolve_from(base_path, p.as_str()),
            None => Ok(base_path.clone()),
        }
    }
}
