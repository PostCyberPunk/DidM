mod copy;
pub use copy::ActionCopy;

mod link;
pub use link::ActionLink;

use anyhow::Result;
use std::path::Path;

pub trait ApplyStrategy {
    fn apply(target: &Path, source: &Path) -> Result<()>;
    fn hint() -> &'static str;
}
