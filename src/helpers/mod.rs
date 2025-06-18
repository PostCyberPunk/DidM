mod checker;
pub use checker::Checker;

mod path;
pub use path::PathExtension;
pub use path::PathResolver;
pub use path::ResolvedPath;

mod prompt;

use crate::model::CheckConfig;

#[derive(Debug)]
pub struct Helpers {
    pub checker: Checker,
}
impl Helpers {
    pub fn new(check_config: &CheckConfig) -> Self {
        Helpers {
            checker: Checker::new(*check_config),
        }
    }
}
