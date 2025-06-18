use super::{super::prompt::confirm, ResolvedPath};
use crate::helpers::path::PathError;
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use path_absolutize::Absolutize;
use std::{collections::HashMap, env, path::PathBuf};

static ENV_VARS: Lazy<HashMap<String, String>> = Lazy::new(|| env::vars().collect());

//REFT: should be use association function
//get config from static once_cell
//get_bool or false
#[derive(Debug)]
pub struct PathResolver {
    check_env: bool,
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
        //FIX:this could be expansive
        for (key, value) in ENV_VARS.iter() {
            let placeholder = format!("${}", key);
            expand = expand.replace(&placeholder, value);
        }
        if self.check_env && expand.contains("$") {
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
        match pathbuf.absolutize() {
            Ok(p) => Ok(p.to_path_buf()),
            Err(e) => Err(PathError::Unknown(e.to_string()).into()),
        }
    }
    // -----------Public ----------------
    pub fn resolve(&self, path: &str, should_check_exist: bool) -> Result<ResolvedPath> {
        let resolve = self
            .expand_tilde(path.to_string())
            .and_then(|p| self.expand_env_vars(p))
            .and_then(|p| self.check_symlink_then_absolute(&p))
            .with_context(|| PathError::ResolveFailed(path.to_string()))?;
        match (should_check_exist, resolve.exists()) {
            (true, false) => Err(PathError::NotExists(resolve).into()),
            _ => Ok(ResolvedPath::new(resolve, path.to_string())),
        }
    }
    pub fn resolve_from(
        &self,
        base_path: &ResolvedPath,
        path: &str,
        should_check_exist: bool,
    ) -> Result<ResolvedPath> {
        let resolved = self.resolve(path, should_check_exist)?;
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
            Some(p) => self.resolve_from(base_path, p.as_str(), true),
            None => Ok(base_path.clone()),
        }
    }
}
