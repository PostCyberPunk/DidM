use super::prompt::confirm;
use crate::model::CheckConfig;
use anyhow::Result;
use std::path::Path;
use thiserror::Error;

//REFT: could be use association function
//get config from static once_cell or failed
#[derive(Debug)]
pub struct Checker {
    config: CheckConfig,
}

impl Checker {
    pub fn new(config: CheckConfig) -> Self {
        Checker { config }
    }
    pub fn is_git_workspace(&self, path: &Path) -> Result<()> {
        if self.config.is_git_workspace {
            return Ok(());
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
    pub fn target_exisit_or_create(&self, path: &Path) -> Result<()> {
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
    pub fn working_dir_is_symlink(&self, path_raw: &str) -> Result<()> {
        if !self.config.is_working_dir_symlink {
            return Ok(());
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
