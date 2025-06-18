use std::{fs, path::Path};

use crate::{helpers::PathExtension, model::profile::Mode};

use super::{AllEntries, Entry, error::EntryApplyError};
use anyhow::{Context, Result};

impl<'a> AllEntries<'a> {
    pub fn apply_list(&self, mode: Mode) {
        let logger = self.logger;
        let list = match mode {
            Mode::Copy => &self.copy_list,
            Mode::Symlink => &self.link_list,
        };
        for entry in list.iter() {
            let target = &entry.target_path;
            let source = &entry.source_path;
            let is_dryrun = self.is_dryrun;

            // TODO: if is_dryrun and should_backup is true,we should handle hit differently
            // match (self.behaviour.should_backup(),self.is_dryrun,self.behaviour.overwrite_existed.unwrap(),hit)
            if target.exists() {
                if entry.overwrite_existed {
                    let _ = std::fs::remove_file(target);
                    //Make sure the file is deleted or Skipped
                    if target.exists() {
                        logger.error(&format!("Overwrite failed, Skipped : {}", target.display()));
                        continue;
                    }
                } else {
                    logger.warn(&format!("Skipped existed file: {}", target.display()));
                    continue;
                }
            }

            if !is_dryrun {
                let result = target.ensure_parent_exists();
                if let Err(e) = result {
                    logger.error(&format!(
                        "Fail to create parent folder,Skipped {} \nReason:{}",
                        target.display(),
                        e
                    ));
                }
                let result = match mode {
                    Mode::Copy => copy_entry(target, source),
                    Mode::Symlink => link_entry(target, source),
                };
                if let Err(e) = result {
                    logger.error(&format!("Skipped {} \nReason:{}", target.display(), e));
                    continue;
                }
            }
            match mode {
                Mode::Copy => {
                    logger.info(&format!(
                        "Copied {} ->\n        {}",
                        target.display(),
                        source.display()
                    ));
                }
                Mode::Symlink => {
                    logger.info(&format!(
                        "Linked {} ->\n        {}",
                        target.display(),
                        source.display()
                    ));
                }
            }
        }
    }
}
fn link_entry(target: &Path, source: &Path) -> Result<()> {
    //HACK: os specific
    let result = std::os::unix::fs::symlink(source, target);
    match result {
        Err(e) => {
            Err(EntryApplyError::FailToCreateLink(target.to_path_buf(), e.to_string()).into())
        }
        _ => Ok(()),
    }
}
fn copy_entry(target: &Path, source: &Path) -> Result<()> {
    //FIX:⬆️⬆️⬆️⬆️⬆️⬆️⬆️⬆️⬆️

    match source.is_dir() {
        //TODO: should remove this,
        //but maybe we could use this for switcher if swticher is a folder
        true => {
            //FIX: oh...recursive function is just a curse
            //TODO:: use fs_extra?
            return Err(EntryApplyError::CantCopyFolder.into());
            // fs::create_dir(target)?;
            // for sub_entry in fs::read_dir(source)? {
            //     let sub_entry = sub_entry?;
            //     let sub_source = sub_entry.path();
            //     let sub_target = target.join(sub_entry.file_name());
            //     let result = copy_entry(&sub_source, &sub_target);
            //     match result {
            //         Ok(()) => continue,
            //         Err(e) => {
            //             return Err(EntryApplyError::FailToCopyFolder(
            //                 target.to_path_buf(),
            //                 e.to_string(),
            //             )
            //             .into());
            //         }
            //     }
            // }
        }
        false => {
            fs::copy(source, target)?;
            // FIX:
            // logger.info(&format!(
            //     "Copied {} ->\n        {}",
            //     target.display(),
            //     source.display()
            // ));
        }
    }
    Ok(())
}
