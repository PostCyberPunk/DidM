use std::path::Path;

use crate::cli::prompt::confirm;
pub fn is_git_repo(path: &Path) -> bool {
    path.join(".git").exists()
}
pub fn check_target(path: &Path) -> bool {
    !path.exists()
        && confirm(&format!(
            "Target Path not exists: \n\
            {}\n\
            Do you want to create it?",
            path.display()
        ))
}
