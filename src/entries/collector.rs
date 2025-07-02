use crate::{
    bakcup::BackupManager,
    entries::DirWalker,
    model::{Behaviour, Sketch, sketch::Mode},
    utils::{Checker, PathResolver, ResolvedPath},
};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::info;

use super::{
    EntriesManager, Entry, EntryBuilderCtx,
    entry_builder::{CollectResult, ExtraBuilder, NormalBuilder, SameSourceBuilder},
};

pub struct EntryCollector<'a> {
    sketch: &'a Sketch,
    variants: &'a Vec<String>,
    builder_ctx: EntryBuilderCtx<'a>,
    entries_manager: &'a mut EntriesManager,
    is_dryrun: bool,
}
//TODO: we need add logs
impl<'a> EntryCollector<'a> {
    pub fn new(
        entries_manager: &'a mut EntriesManager,
        sketch: &'a Sketch,
        base_path: &ResolvedPath,
        variants: &'a Vec<String>,
        sketch_name: &str,
        behaviour: &Behaviour,
        backup_manager: Option<&'a BackupManager>,
    ) -> Result<Self> {
        info!("Generating entries for `{}` ...", sketch_name);

        let is_dryrun = entries_manager.is_dryrun;
        let overwrite_existed = behaviour.overwrite_existed.unwrap();

        //Reoslve Path
        let source_root =
            PathResolver::resolve_from_with_ctx(base_path, &sketch.source_path, "source", true)?;
        let target_root =
            PathResolver::resolve_from_with_ctx(base_path, &sketch.target_path, "target", false)?;

        //Check target exist
        let exist = Checker::target_exisit_or_create(target_root.as_path())?;
        if !exist && !is_dryrun {
            std::fs::create_dir_all(target_root.as_path())?;
        }

        //prepare entry builder context
        let builder_ctx = EntryBuilderCtx {
            source_root,
            target_root,
            backup_manager,
            overwrite: overwrite_existed,
        };
        Ok(Self {
            entries_manager,
            variants,
            builder_ctx,
            sketch,
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
            "null file",
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
            "empty file",
        )
        .context("Failed to get empty entries")?;
        //Get extra entries
        self.get_extra_entris(self.sketch)
            .context("Failed to get extra entries")?;
        //Done!
        Ok(())
    }

    //TODO: doesnt handle backup,maybe add a state to entry?
    fn add_entry(&mut self, entry: Entry, result: CollectResult, mode: Mode) {
        if let CollectResult::SkipWithError(err) = result {
            self.entries_manager.add_error((entry, err))
        } else if result == CollectResult::Skip {
            self.entries_manager.skip_entry(entry);
        } else {
            match mode {
                Mode::Copy => self.entries_manager.add_copy(entry),
                Mode::Symlink => self.entries_manager.add_link(entry),
            }
        }
    }

    fn get_normal_entries(&mut self) -> Result<()> {
        let source_paths = DirWalker::new(
            self.sketch,
            self.variants,
            self.builder_ctx.source_root.as_path(),
        )
        .get_walker()?
        .run()?;
        for source_path in source_paths {
            let (entry, result) = NormalBuilder::create(&self.builder_ctx, source_path)?.build();
            self.add_entry(entry, result, self.sketch.mode);
        }
        Ok(())
    }
    fn collect_same_source(
        &mut self,
        paths: &[String],
        source_path: &Path,
        mode: Mode,
        hint: &str,
    ) -> Result<()> {
        for path in paths.iter() {
            let (entry, result) = SameSourceBuilder::create(
                &self.builder_ctx,
                source_path.to_path_buf(),
                path,
                hint,
            )?
            .build();
            self.add_entry(entry, result, mode);
        }
        Ok(())
    }
    fn get_extra_entris(&mut self, sketch: &Sketch) -> Result<()> {
        for extra in sketch.extra_entries.iter() {
            let (entry, result) =
                ExtraBuilder::create(&self.builder_ctx, &extra.source_path, &extra.target_path)?
                    .build();
            self.add_entry(entry, result, extra.mode.unwrap_or(sketch.mode));
        }
        Ok(())
    }
}
