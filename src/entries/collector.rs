use crate::{
    bakcup::{BackupManager, BackupState},
    entries::DirWalker,
    model::{Behaviour, Sketch, sketch::Mode},
    utils::{Checker, PathResolver, ResolvedPath},
};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::{info, warn};

use super::{EntriesManager, Entry, SouceType};

pub struct EntryCollector<'a> {
    source_root: ResolvedPath,
    target_root: ResolvedPath,
    entries_manager: &'a mut EntriesManager,
    sketch: &'a Sketch,
    backup_manager: Option<BackupManager>,
    overwrite_existed: bool,
    is_dryrun: bool,
}
//TODO: we need add logs
impl<'a> EntryCollector<'a> {
    pub fn new(
        entries_manager: &'a mut EntriesManager,
        sketch: &'a Sketch,
        base_path: &ResolvedPath,
        sketch_name: &str,
        behaviour: &Behaviour,
        backup_manager: Option<BackupManager>,
    ) -> Result<Self> {
        info!("Generating entries for `{}` ...", sketch_name);

        let is_dryrun = entries_manager.is_dryrun;
        let overwrite_existed = behaviour.overwrite_existed.unwrap();

        //Reoslve Path
        let source_root = Self::resolve_path(base_path, &sketch.source_path, "source", true)?;
        let target_root = Self::resolve_path(base_path, &sketch.target_path, "target", false)?;

        //Check target exist
        let exist = Checker::target_exisit_or_create(target_root.get())?;
        if !exist && !is_dryrun {
            std::fs::create_dir_all(target_root.get())?;
        }
        Ok(Self {
            entries_manager,
            source_root,
            target_root,
            sketch,
            backup_manager,
            overwrite_existed,
            is_dryrun,
        })
    }
    pub fn collect(mut self) -> Result<()> {
        //Get Normal Entries
        self.get_normal_entries()
            .context("Failed to get normal entries")?;
        //Get null entries
        //FIX:that looks like pretty fucked up
        let dev_null = PathBuf::from("/dev/null");
        self.collect_same_source(
            &self.sketch.null_files,
            &dev_null,
            Mode::Symlink,
            SouceType::Null,
        )
        .context("Failed to get null entries")?;
        //Get empty entries
        //FIX: os .bad practice
        let _empty_tmp = PathBuf::from("/tmp/didm_empty");
        if !self.is_dryrun && !&self.sketch.empty_files.is_empty() {
            std::fs::write(_empty_tmp.clone(), "")?;
        }
        self.collect_same_source(
            &self.sketch.empty_files,
            &_empty_tmp,
            Mode::Copy,
            SouceType::Empty,
        )
        .context("Failed to get empty entries")?;
        //Get extra entries
        self.get_extra_entris(self.sketch)
            .context("Failed to get extra entries")?;
        //Done!
        Ok(())
    }

    fn add_entry(&mut self, mut entry: Entry, mode: Mode, s_type: SouceType) {
        if s_type != SouceType::Normal {
            entry.bakcup_state = match &self.backup_manager {
                None => BackupState::Ok,
                Some(bakcuper) => match bakcuper.backup_other(&entry.target_path, s_type) {
                    Ok(s) => s,
                    _ => BackupState::Skip,
                },
            };
        }
        // self.entries_manager.add_entry(mode, entry);
    }
    fn resolve_path(
        base_path: &ResolvedPath,
        path: &str,
        ctx: &str,
        should_check_exist: bool,
    ) -> Result<ResolvedPath> {
        let result = PathResolver::resolve_from(base_path, path, should_check_exist)
            .with_context(|| format!("Invalid {} path: {}", ctx, path))?;
        info!("{} path: {}", ctx, result.di_string());
        Ok(result)
    }

    fn get_normal_entries(&mut self) -> Result<()> {
        let source_paths = DirWalker::new(self.sketch, self.source_root.get())
            .get_walker()?
            .run()?;
        for source_path in source_paths {
            let relative_path = match source_path.strip_prefix(self.source_root.get()) {
                Ok(p) => p,
                Err(e) => {
                    warn!("Invalid entry path: {}", e);
                    continue;
                }
            };
            let target_path = self.target_root.get().join(relative_path);
            // BAKCUP
            let bakcup_state = match &self.backup_manager {
                None => BackupState::Ok,
                Some(bakcuper) => match bakcuper.backup_normal(&target_path, relative_path) {
                    Ok(s) => s,
                    _ => BackupState::Skip,
                },
            };
            //FIX: this won't backup target
            let mut entry = Entry::new(source_path, target_path, self.overwrite_existed);
            entry.bakcup_state = bakcup_state;
            self.add_entry(entry, self.sketch.mode, SouceType::Normal);
        }
        Ok(())
    }
    fn collect_same_source(
        &mut self,
        paths: &[String],
        source_path: &Path,
        mode: Mode,
        s_type: SouceType,
    ) -> Result<()> {
        for path in paths.iter() {
            let rp = PathResolver::resolve_from(&self.target_root, path, false);
            let entry = match rp {
                Err(err) => {
                    warn!("Skipping entry:{}\nCasuse:{}", path, err);
                    continue;
                }
                Ok(target_path) => {
                    if target_path.get().exists() {
                        info!("(null/empty) Skipping existed target:{}", path);
                        continue;
                    }
                    Entry::new(source_path.to_path_buf(), target_path.into_pathbuf(), false)
                }
            };
            self.add_entry(entry, mode, s_type);
        }
        Ok(())
    }
    fn get_extra_entris(&mut self, sketch: &Sketch) -> Result<()> {
        for extra in sketch.extra_entries.iter() {
            let e = Entry::new(
                Self::resolve_path(&self.source_root, &extra.source_path, "extra entry", true)?
                    .into_pathbuf(),
                Self::resolve_path(
                    &self.target_root,
                    &extra.target_path,
                    "extra entry target",
                    false,
                )?
                .into_pathbuf(),
                self.overwrite_existed,
            );
            match extra.mode {
                Some(mode) => self.add_entry(e, mode, SouceType::Extra),
                None => self.add_entry(e, sketch.mode, SouceType::Extra),
            }
        }
        Ok(())
    }
}
