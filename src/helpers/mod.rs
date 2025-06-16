mod checker;
mod path;
pub use checker::Checker;
pub use path::PathError;
pub use path::PathResolver;
pub use path::ResolvedPath;

use crate::model::SkipCheck;

pub struct Helpers {
    pub checker: Checker,
    pub path_resolver: PathResolver,
}
impl Helpers {
    pub fn new(check_config: &SkipCheck) -> Self {
        Helpers {
            checker: Checker::new(*check_config),
            path_resolver: PathResolver::new(check_config.unresolved_env),
        }
    }
}
