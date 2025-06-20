use std::{fs, io, path::Path};

use crate::{model::sketch::Mode, utils::PathExtension};

use super::{super::error::EntryApplyError, EntriesManager};
use anyhow::Result;

impl<'a> EntriesManager<'a> {
    pub fn copy_and_link(&self) -> Result<()> {
        self.apply_list(Mode::Symlink);
        self.apply_list(Mode::Copy);
        Ok(())
    }
    fn apply_list(&self, mode: Mode) {
        let logger = self.logger;
        let list = match mode {
            Mode::Copy => &self.entry_list.copy_list,
            Mode::Symlink => &self.entry_list.link_list,
        };
        let mode_hint = match mode {
            Mode::Copy => "Copy",
            Mode::Symlink => "Symlink",
        };
        for entry in list.iter() {
            let target = &entry.target_path;
            let source = &entry.source_path;

            if self.is_dryrun {
                logger.info(&format!(
                    "Dry-run: {}: from {}\n to {}",
                    mode_hint,
                    source.display(),
                    target.display()
                ));
                continue;
            }

            if target.exists() {
                if entry.overwrite_existed {
                    if let Err(e) = remove_target(target) {
                        logger.error(&format!(
                            "Skipped {} \n Failed to remove target, \n reason: {}",
                            target.display(),
                            e
                        ));
                        continue;
                    }
                    if target.exists() {
                        logger.error(&format!(
                            "Skipped {} \n Failed to remove target, \n reason: Unknown,Please Remove it mannually",
                            target.display(),
                        ));
                        continue;
                    }
                } else {
                    logger.warn(&format!("Skipped existed file: {}", target.display()));
                    continue;
                }
            }

            if let Err(e) = target.ensure_parent_exists() {
                logger.error(&format!(
                    "Skippied {} \n Failed to create parent folder,\n reason: {}",
                    target.display(),
                    e
                ));
                continue;
            }

            if let Err(e) = match mode {
                Mode::Copy => copy_entry(target, source),
                Mode::Symlink => link_entry(target, source),
            } {
                logger.error(&format!(
                    "Failed to create {}:from {} \n to {}, reason: {}",
                    mode_hint,
                    source.display(),
                    target.display(),
                    e
                ));
                continue;
            }

            logger.info(&format!(
                "{}:from {}\n to {}",
                mode_hint,
                source.display(),
                target.display()
            ));
        }
    }
}
fn remove_target(target: &Path) -> io::Result<()> {
    if target.is_dir() {
        fs::remove_dir_all(target)
    } else {
        fs::remove_file(target)
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
