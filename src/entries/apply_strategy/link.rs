use super::ApplyStrategy;
use crate::entries::error::EntryApplyError;
use anyhow::Result;
use std::{os::unix::fs::symlink, path::Path};

pub struct ActionLink;

impl ApplyStrategy for ActionLink {
    fn apply(target: &Path, source: &Path) -> Result<()> {
        //HACK:os specific
        symlink(source, target).map_err(|e| {
            EntryApplyError::FailToCreateLink(target.to_path_buf(), e.to_string()).into()
        })
    }
    fn hint() -> &'static str {
        "Link"
    }
}
