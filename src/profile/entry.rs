use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::{
    helpers::{PathExtension, PathResolver, ResolvedPath},
    log::Logger,
    model::{Behaviour, Profile, behaviour, profile::Mode},
};

use super::walk::WalkerContext;

pub struct Entries<'a> {
    pub entries: Vec<Option<(PathBuf, PathBuf)>>,
    // pub extra_entries: Vec<Option<(PathBuf, PathBuf, Mode)>>,
    // pub null_entries: Vec<Option<PathBuf>>,
    // pub empty_entries: Vec<Option<PathBuf>>,
    pub behaviour: &'a Behaviour,
    pub logger: &'a Logger,
    pub mode: Mode,
    pub is_dryrun: bool,
}
impl<'a> Entries<'a> {
    pub fn new(
        source_path: &Path,
        target_path: &Path,
        profile: &Profile,
        behaviour: &'a Behaviour,
        logger: &'a Logger,
        mode: Mode,
        is_dryrun: bool,
    ) -> Result<Self> {
        //Normal Entries
        //Walk director
        let entries = WalkerContext::new(profile, source_path, logger)
            .get_walker()?
            .run()?;
        let entries: Vec<Option<(PathBuf, PathBuf)>> = entries
            .into_iter()
            .map(|entry| {
                let relative_path = match entry.strip_prefix(source_path) {
                    Ok(p) => p,
                    Err(e) => {
                        logger.warn(&format!("Invalid entry path: {}", e));
                        return None;
                    }
                };
                let p = target_path.join(relative_path);
                Some((entry, p))
                //FIX:BAKCUP
                // match backuper.backup(&p, relative_path, logger, || p.exists()) {
                //     Ok(_) => Some((entry, p)),
                //     Err(err) => {
                //         logger.warn(&format!(
                //             "Skipping entry:{}\nCasuse:{}",
                //             entry.display(),
                //             err
                //         ));
                //         None
                //     }
                // }
            })
            .collect();
        Ok(Self {
            entries,
            behaviour,
            logger,
            mode,
            is_dryrun,
        })
    }

    fn collect_path(
        logger: &Logger,
        base_path: &ResolvedPath,
        paths: &[String],
        resolver: &PathResolver,
    ) -> Result<Vec<Option<PathBuf>>> {
        let mut result = Vec::new();
        for path in paths.iter() {
            let rp = resolver.resolve_from(base_path, path);
            let entry = match rp {
                Err(err) => {
                    logger.warn(&format!("Skipping entry:{}\nCasuse:{}", path, err));
                    None
                }
                Ok(p) => Some(p.into_pathbuf()),
            };
            result.push(entry);
        }
        Ok(result)
    }

    pub fn apply_entries(&mut self) -> Result<()> {
        self.entries.iter().for_each(|entry| {
            if let Some((src, tgt)) = entry {
                self.apply_entry(src, tgt).unwrap();
            }
        });
        Ok(())
    }

    fn apply_entry(&self, src: &Path, tgt: &Path) -> Result<()> {
        let logger = self.logger;
        let mode = &self.mode;
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
