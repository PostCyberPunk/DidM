use crate::{
    bakcup::BackupManager,
    entries::DirWalker,
    model::{Behaviour, Sketch, sketch::Mode},
    utils::{Checker, PathResolver, ResolvedPath},
};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::{info, warn};

use super::{EntriesManager, Entry, EntryBuilder, EntryBuilderCtx, SouceType};

pub struct EntryCollector<'a> {
    source_root: ResolvedPath,
    target_root: ResolvedPath,
    sketch: &'a Sketch,
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
            source_root: source_root.clone(),
            target_root: target_root.clone(),
            backup_manager,
            overwrite: overwrite_existed,
        };
        Ok(Self {
            entries_manager,
            source_root,
            target_root,
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

    fn add_entry(&mut self, entry: Entry, mode: Mode) {
        match mode {
            Mode::Copy => self.entries_manager.add_copy(entry),
            Mode::Symlink => self.entries_manager.add_link(entry),
        }
    }

    fn get_normal_entries(&mut self) -> Result<()> {
        let source_paths = DirWalker::new(self.sketch, self.source_root.as_path())
            .get_walker()?
            .run()?;
        for source_path in source_paths {
            //TODO: intead of relative path we should source_root and this logic
            // let relative_path = match source_path.strip_prefix(self.source_root.as_path()) {
            //     Ok(p) => p.to_path_buf(),
            //     Err(e) => {
            //         warn!("Invalid entry path: {}", e);
            //         continue;
            //     }
            // };
            // let entry = EntryBuilder::new(
            //     source_path,
            //     self.target_root.clone().into_pathbuf(),
            //     &self.builder_ctx,
            // )
            // .source_type(SouceType::Normal)
            // .relative_path(relative_path)
            // .build()
            // .await?;
            // self.add_entry(entry, self.sketch.mode);
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
                    if target_path.as_path().exists() {
                        info!("(null/empty) Skipping existed target:{}", path);
                        continue;
                    }
                    // EntryBuilder::new(
                    //     source_path.to_path_buf(),
                    //     target_path.into_pathbuf(),
                    //     &self.builder_ctx,
                    // )
                    // .source_type(s_type)
                    // .overwrite(false)
                    // .build()
                    // .await?
                }
            };
            // self.add_entry(entry, mode);
        }
        Ok(())
    }
    fn get_extra_entris(&mut self, sketch: &Sketch) -> Result<()> {
        for extra in sketch.extra_entries.iter() {
            let source_path = PathResolver::resolve_from_with_ctx(
                &self.source_root,
                &extra.source_path,
                "extra entry",
                true,
            )?
            .into_pathbuf();
            let target_path = PathResolver::resolve_from_with_ctx(
                &self.target_root,
                &extra.target_path,
                "extra entry target",
                false,
            )?
            .into_pathbuf();
            // let entry = EntryBuilder::new(source_path, target_path, &self.builder_ctx)
            //     .source_type(SouceType::Extra)
            //     .build()
            //     .await?;
            // match extra.mode {
            //     Some(mode) => self.add_entry(entry, mode),
            //     None => self.add_entry(entry, sketch.mode),
            // }
        }
        Ok(())
    }
}
