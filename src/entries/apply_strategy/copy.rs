use super::ApplyStrategy;
use crate::entries::error::EntryApplyError;
use anyhow::Result;
use std::{fs, path::Path};

pub struct ActionCopy;

impl ApplyStrategy for ActionCopy {
    fn apply(target: &Path, source: &Path) -> Result<()> {
        if source.is_dir() {
            //FIX: maybe use fx_extra
            Err(EntryApplyError::CantCopyFolder.into())
        } else {
            fs::copy(source, target)?;
            Ok(())
        }
    }
    fn hint() -> &'static str {
        "Copy"
    }
}
