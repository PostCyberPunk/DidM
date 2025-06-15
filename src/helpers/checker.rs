use std::path::Path;

use crate::{cli::prompt::confirm, model::SkipCheck};

pub struct Checker {
    pub config: SkipCheck,
}

impl Checker {
    pub fn new(config: SkipCheck) -> Self {
        Checker { config }
    }
    pub fn check_git_repo(&self, path: &Path) -> bool {
        path.join(".git").exists() || confirm("This is not a git repo, continue?")
    }
    pub fn check_target(&self, path: &Path) -> bool {
        path.exists()
            || confirm(&format!(
                "Target Path not exists: \n\
            {}\n\
            Do you want to create it?",
                path.display()
            ))
    }
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
