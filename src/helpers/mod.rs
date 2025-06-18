mod checker;
mod path;
mod prompt;
pub use checker::Checker;
pub use path::PathExtension;
pub use path::PathResolver;
pub use path::ResolvedPath;

use crate::model::CheckConfig;

pub struct Helpers {
    pub checker: Checker,
    pub path_resolver: PathResolver,
}
impl Helpers {
    pub fn new(check_config: &CheckConfig) -> Self {
        Helpers {
            checker: Checker::new(*check_config),
            path_resolver: PathResolver::new(!check_config.unresolved_env),
        }
    }
}
