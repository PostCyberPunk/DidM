use super::{AllEntries, Entry};
use crate::{
    helpers::{Helpers, ResolvedPath},
    log::Logger,
    model::{Behaviour, Profile, profile::Mode},
    profile::WalkerContext,
};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

impl<'a> AllEntries<'a> {
    pub fn new(helpers: &'a Helpers, logger: &'a Logger, is_dryrun: bool) -> Self {
        Self {
            copy_list: Vec::new(),
            link_list: Vec::new(),
            helpers,
            logger,
            is_dryrun,
            // overwrite_existed,
        }
    }

    fn resolve_path(
        &self,
        base_path: &ResolvedPath,
        path: &str,
        ctx: &str,
    ) -> Result<ResolvedPath> {
        let result = self
            .helpers
            .path_resolver
            .resolve_from(base_path, path)
            .with_context(|| format!("Invalid {} path: {}", ctx, path))?;
        self.logger
            .info(&format!("{} path: {}", ctx, result.di_string()));
        Ok(result)
    }

    fn get_normal_entries(
        &mut self,
        profile: &Profile,
        source_root: &ResolvedPath,
        target_root: &ResolvedPath,
        overwrite_existed: bool,
    ) -> Result<()> {
        let target_list = match profile.mode {
            Mode::Symlink => &mut self.link_list,
            Mode::Copy => &mut self.copy_list,
        };
        let source_paths = WalkerContext::new(profile, source_root.get(), self.logger)
            .get_walker()?
            .run()?;
        for source_path in source_paths {
            let relative_path = match source_path.strip_prefix(source_root.get()) {
                Ok(p) => p,
                Err(e) => {
                    self.logger.warn(&format!("Invalid entry path: {}", e));
                    continue;
                }
            };
            let target_path = target_root.get().join(relative_path);
            target_list.push(Entry::new(source_path, target_path, overwrite_existed));
        }
        Ok(())
    }

    fn collect_same_source(
        &mut self,
        paths: &[String],
        base_path: &ResolvedPath,
        source_path: &Path,
        overwrite_existed: bool,
        mode: Mode,
    ) -> Result<()> {
        for path in paths.iter() {
            let rp = self.helpers.path_resolver.resolve_from(base_path, path);
            let entry = match rp {
                Err(err) => {
                    self.logger
                        .warn(&format!("Skipping entry:{}\nCasuse:{}", path, err));
                    continue;
                }
                Ok(target_path) => Entry::new(
                    source_path.to_path_buf(),
                    target_path.into_pathbuf(),
                    overwrite_existed,
                ),
            };
            match mode {
                Mode::Symlink => self.link_list.push(entry),
                Mode::Copy => self.copy_list.push(entry),
            }
        }
        Ok(())
    }

    fn get_extra_entris(
        &mut self,
        profile: &Profile,
        source_root: &ResolvedPath,
        target_root: &ResolvedPath,
        overwrite_existed: bool,
    ) -> Result<()> {
        for extra in profile.extra_entries.iter() {
            let e = Entry::new(
                self.resolve_path(source_root, &extra.source_path, "extra entry")?
                    .into_pathbuf(),
                self.resolve_path(target_root, &extra.target_path, "extra entry target")?
                    .into_pathbuf(),
                overwrite_existed,
            );
            match &extra.mode {
                Some(mode) => match mode {
                    Mode::Copy => self.copy_list.push(e),
                    Mode::Symlink => self.link_list.push(e),
                },
                None => match profile.mode {
                    Mode::Copy => self.copy_list.push(e),
                    Mode::Symlink => self.link_list.push(e),
                },
            }
        }
        Ok(())
    }

    //TODO: we need add logs
    pub fn add_profile(
        &mut self,
        profile: &Profile,
        base_path: &ResolvedPath,
        behaviour: &Behaviour,
        profile_name: &str,
    ) -> Result<()> {
        let logger = self.logger;
        let should_backup = behaviour.should_backup();
        let overwrite_existed = behaviour.overwrite_existed.unwrap();

        logger.info(&format!("Generating entries for `{}` ...", profile_name));
        //Reoslve Path
        let source_root = self.resolve_path(base_path, &profile.source_path, "source")?;
        let target_root = self.resolve_path(base_path, &profile.target_path, "target")?;
        //FIX:this will never work since we are using canonicalize
        self.helpers
            .checker
            .target_exisit_or_create(target_root.get())?;

        //Get Normal Entries
        self.get_normal_entries(profile, &source_root, &target_root, overwrite_existed)
            .context("Failed to get normal entries")?;
        //Get null entries
        //FIX:that looks like pretty fucked up
        let dev_null = PathBuf::from("/dev/null");
        self.collect_same_source(
            &profile.null_files,
            &source_root,
            &dev_null,
            overwrite_existed,
            Mode::Symlink,
        )
        .context("Failed to get null entries")?;
        //Get empty entries
        self.collect_same_source(
            &profile.empty_files,
            &source_root,
            &dev_null,
            overwrite_existed,
            Mode::Copy,
        )
        .context("Failed to get empty entries")?;
        //Get extra entries
        self.get_extra_entris(profile, &source_root, &target_root, overwrite_existed)
            .context("Failed to get extra entries")?;
        //Done!
        Ok(())
    }
}
