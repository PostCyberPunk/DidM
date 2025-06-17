use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::{
    helpers::PathExtension,
    log::Logger,
    model::{Behaviour, profile::Mode},
};

pub struct Entries<'a> {
    pub entries: Vec<Option<(PathBuf, PathBuf)>>,
    pub behaviour: &'a Behaviour,
    pub logger: &'a Logger,
    pub mode: &'a Mode,
    pub is_dryrun: bool,
}
impl<'a> Entries<'a> {
    pub fn new(
        entries: Vec<Option<(PathBuf, PathBuf)>>,
        behaviour: &'a Behaviour,
        logger: &'a Logger,
        mode: &'a Mode,
        is_dryrun: bool,
    ) -> Self {
        Self {
            entries,
            behaviour,
            logger,
            mode,
            is_dryrun,
        }
    }

    pub fn apple_entries(&mut self) -> Result<()> {
        self.entries.iter().for_each(|entry| {
            if let Some((src, tgt)) = entry {
                self.apply_entry(src, tgt).unwrap();
            }
        });
        Ok(())
    }

    fn apply_entry(&self, src: &Path, tgt: &Path) -> Result<()> {
        let logger = self.logger;
        let mode = self.mode;
        let hit = tgt.exists();
        let is_dryrun = self.is_dryrun;

        // TODO: if is_dryrun and should_backup is true,we should handle hit differently
        // so we should set up closure when initializing,then call closure here
        // match (self.behaviour.should_backup(),self.is_dryrun,self.behaviour.overwrite_existed.unwrap(),hit)
        if !self.behaviour.overwrite_existed.unwrap() && hit {
            logger.warn(&format!("Skipped existed file: {}", tgt.display()));
            return Ok(());
        }
        if !is_dryrun {
            tgt.ensure_parent_exists()?;
        }
        match mode {
            Mode::Symlink => {
                if !is_dryrun {
                    let _ = std::fs::remove_file(tgt);
                    //Make sure the file is deleted or Skipped
                    if tgt.exists() {
                        logger.error(&format!("Overwrite failed, Skipped : {}", tgt.display()));
                        return Ok(());
                    }
                    //HACK: os specific
                    std::os::unix::fs::symlink(src, tgt)
                        .with_context(|| format!("symlink {:?} ->\n        {:?}", src, tgt))?;
                }
                logger.info(&format!(
                    "Symlinking {} ->\n        {}",
                    tgt.display(),
                    src.display(),
                ));
            }
            Mode::Copy => match src.is_dir() {
                //TODO: should remove this,
                //but maybe we could use this for switcher if swticher is a folder
                true => {
                    //TODO:: use fs_extra?
                    if !is_dryrun {
                        fs::create_dir_all(tgt)?;
                    }
                    for entry in fs::read_dir(src)? {
                        let entry = entry?;
                        let sub_src = entry.path();
                        let sub_tgt = tgt.join(entry.file_name());
                        self.apply_entry(&sub_src, &sub_tgt)?;
                    }
                }
                false => {
                    if !is_dryrun {
                        let _ = std::fs::remove_file(tgt);
                        //Make sure the file is deleted or Skipped
                        if tgt.exists() {
                            logger.error(&format!("Overwrite failed, Skipped : {}", tgt.display()));
                            return Ok(());
                        }
                        fs::copy(src, tgt)?;
                    }
                    logger.info(&format!(
                        "Copied {} ->\n        {}",
                        tgt.display(),
                        src.display()
                    ));
                }
            },
        }
        Ok(())
    }
}
