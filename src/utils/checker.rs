use super::prompt::confirm;
use crate::{config::CHCECK_CONFIG, model::CheckConfig};
use anyhow::Result;
use std::path::Path;
use thiserror::Error;

#[derive(Debug)]
pub struct Checker {}
impl Checker {
    fn get_config() -> Option<&'static CheckConfig> {
        match CHCECK_CONFIG.get() {
            Some(c) => Some(c),
            None => {
                None
                //TODO: logger here
            }
        }
    }
    pub fn is_git_workspace(path: &Path) -> Result<()> {
        if let Some(c) = Self::get_config() {
            if c.is_git_workspace {
                return Ok(());
            }
        }
        if path.join(".git").exists()
            || confirm(&format!(
                "Current Path: {}\n\
            This is not a git repo, continue?",
                path.display()
            ))
        {
            Ok(())
        } else {
            Err(CheckError::NotGitRepo.into())
        }
    }
    pub fn target_exisit_or_create(path: &Path) -> Result<()> {
        if path.exists()
            || confirm(&format!(
                "Target Path not exists: \n\
            {}\n\
            Do you want to create it?",
                path.display()
            ))
        {
            Ok(())
        } else {
            Err(CheckError::TargetPathNotExists.into())
        }
    }
    pub fn working_dir_is_symlink(path_raw: &str) -> Result<()> {
        if let Some(c) = Self::get_config() {
            if c.is_working_dir_symlink {
                return Ok(());
            }
        }
        let path = Path::new(path_raw);
        if path.is_symlink()
            && !confirm(&format!(
                "config located at: {}\n\
                    which is a symlink,this may lead to some unexcepted issue\n\
                    do you want to continue?",
                path.display()
            ))
        {
            return Err(CheckError::WorkingDirectoryIsSymlink.into());
        }
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum CheckError {
    #[error("Not a git repo")]
    NotGitRepo,
    #[error("Target path not exists")]
    TargetPathNotExists,
    #[error("Working directory is symlink")]
    WorkingDirectoryIsSymlink,
}

//NOTE: macro is not lazy
// pub fn check_target(path: &Path) -> bool {
//     let hint = &format!(
//         "Target Path not exists: \n\
//             {}\n\
//             Do you want to create it?",
//         path.display()
//     );
//     path.exists() || confirm(hint)
// }
