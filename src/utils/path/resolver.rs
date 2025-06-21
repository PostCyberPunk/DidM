use super::{super::prompt::confirm, ResolvedPath};
use crate::{config::CHCECK_CONFIG, utils::path::PathError};
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use path_absolutize::Absolutize;
use std::{collections::HashMap, env, path::PathBuf};

static ENV_VARS: Lazy<HashMap<String, String>> = Lazy::new(|| env::vars().collect());

#[derive(Debug)]
pub struct PathResolver {}
impl PathResolver {
    fn should_check_env() -> bool {
        match CHCECK_CONFIG.get() {
            //TODO: flip flag is anooying
            Some(c) => !c.unresolved_env,
            None => true,
        }
    }
    // -----------Internal------
    fn expand_env_vars(path: String) -> Result<String> {
        if !path.contains("$") {
            return Ok(path);
        }
        let mut expand = path;
        //FIX:this could be expansive
        for (key, value) in ENV_VARS.iter() {
            let placeholder = format!("${}", key);
            expand = expand.replace(&placeholder, value);
        }
        if Self::should_check_env() && expand.contains("$") {
            return Err(PathError::EnvVarMissing(expand).into());
        }
        Ok(expand)
    }
    fn expand_tilde(path: String) -> Result<String> {
        if path.starts_with("~") {
            let home = env::var("HOME").map_err(|_| {
                PathError::EnvVarMissing("Failed to resolve `~` from $HOME ".to_string())
            })?;
            let result = path.replacen("~", &home, 1);
            return Ok(result);
        }
        Ok(path)
    }
    fn check_symlink_then_absolute(path: &str) -> Result<PathBuf> {
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
    pub fn resolve(path: &str, should_check_exist: bool) -> Result<ResolvedPath> {
        let resolve = Self::expand_tilde(path.to_string())
            .and_then(Self::expand_env_vars)
            .and_then(|p| Self::check_symlink_then_absolute(&p))
            .with_context(|| PathError::ResolveFailed(path.to_string()))?;
        match (should_check_exist, resolve.exists()) {
            (true, false) => Err(PathError::NotExists(resolve).into()),
            _ => Ok(ResolvedPath::new(resolve, path.to_string())),
        }
    }
    pub fn resolve_from(
        base_path: &ResolvedPath,
        path: &str,
        should_check_exist: bool,
    ) -> Result<ResolvedPath> {
        if !Self::is_unresolved_absolute(path) {
            return base_path.to_child(path, should_check_exist);
        }
        Self::resolve(path, should_check_exist)
        // if resolved.get().is_absolute() {
        //     Ok(resolved)
        // } else {
        //     Ok(ResolvedPath::new(
        //         base_path.get().join(resolved.get()),
        //         path.to_string(),
        //     ))
        // }
    }
    pub fn resolve_from_or_base(
        base_path: &ResolvedPath,
        path: &Option<String>,
    ) -> Result<ResolvedPath> {
        match path {
            Some(p) => Self::resolve_from(base_path, p.as_str(), true),
            None => Ok(base_path.clone()),
        }
    }
    fn is_unresolved_absolute(s: &str) -> bool {
        s.starts_with("$") || s.starts_with("~") || s.starts_with("/")
    }
}
