use anyhow::Result;
use std::path::Path;
use thiserror::Error;

use crate::{cli::prompt::confirm, model::SkipCheck};

pub struct Checker {
    pub config: SkipCheck,
}

impl Checker {
    pub fn new(config: SkipCheck) -> Self {
        Checker { config }
    }
    pub fn check_git_repo(&self, path: &Path) -> Result<()> {
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
    pub fn check_target(&self, path: &Path) -> Result<()> {
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
}

#[derive(Error, Debug)]
pub enum CheckError {
    #[error("Not a git repo")]
    NotGitRepo,
    #[error("Target path not exists")]
    TargetPathNotExists,
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
