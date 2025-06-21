use crate::log::Logger;
use crate::model::sketch::{Mode, Sketch, Unit};
use anyhow::{Context, Result};
use ignore::WalkBuilder;
use ignore::overrides::OverrideBuilder;
use std::path::{Path, PathBuf};

//TODO: can't kill self,
//this should be oneshot,so init and run then dead
pub struct DirWalker<'a> {
    walker: Option<WalkBuilder>,
    base_path: &'a Path,
    logger: &'a Logger,
    ignore: &'a Vec<String>,
    respect_gitignore: bool,
    ignore_hidden: bool,
    only_ignore: bool,
    unit: Unit,
    mode: Mode,
}

impl<'a> DirWalker<'a> {
    pub fn new(sketch: &'a Sketch, base_path: &'a Path, logger: &'a Logger) -> Self {
        Self {
            walker: None,
            logger,
            base_path,
            ignore: &sketch.ignore,
            respect_gitignore: sketch.respect_gitignore,
            ignore_hidden: sketch.ignore_hidden,
            only_ignore: sketch.only_ignore,
            unit: sketch.unit,
            mode: sketch.mode,
        }
    }

    pub fn get_walker(&mut self) -> Result<&Self> {
        let base_path = self.base_path;

        let mut walker = WalkBuilder::new(base_path);

        walker.add_custom_ignore_filename("didmignore");

        let mut overrides = OverrideBuilder::new(base_path);

        overrides.add("!didm.toml")?;
        overrides.add("!.gitignore")?;
        overrides.add("!didmignore")?;
        overrides.add("!didm_backup")?;

        let _prefix = if self.only_ignore { "" } else { "!" };

        if self.only_ignore && self.ignore.is_empty() {
            overrides
                .add("!")
                .context(format!("Failed to make `only_ignore` happen"))?;
        }
        for ignore_item in self.ignore {
            overrides
                .add(&format!("{}{}", _prefix, ignore_item))
                .context(format!("Failed to add ignore item:{}", ignore_item))?;
        }
        if self.unit == Unit::Dir && self.mode == Mode::Symlink {
            walker.max_depth(Some(1));
        };
        walker
            .hidden(self.ignore_hidden)
            .git_ignore(self.respect_gitignore)
            .overrides(overrides.build()?);

        self.walker = Some(walker);
        Ok(self)
    }
    pub fn run(&self) -> Result<Vec<PathBuf>> {
        let logger = self.logger;
        let walker = self.walker.as_ref().ok_or_else(|| {
            logger.error("Worker not initialized");
            anyhow::anyhow!("Failed to get walker, issue this with log and your configuration")
        })?;

        let mut entries = Vec::new();
        for result in walker.build() {
            let entry = result.context("Failed to get entry")?;
            let entry_type = entry.file_type().unwrap();
            //NOTE: we can add a flag ,let user know deal with symlink is a bad idea
            //then do walk_symblink && is_symlink() || ...
            let unit_condition = match (self.unit, self.mode) {
                (Unit::Dir, Mode::Symlink) => entry_type.is_dir() || entry_type.is_file(),
                _ => entry_type.is_file(),
            };
            if unit_condition {
                entries.push(entry.path().to_path_buf());
            }
        }
        //FIX: the result always return the base_path as first entry?
        if self.unit == Unit::Dir && self.mode == Mode::Symlink {
            entries.remove(0);
        }
        Ok(entries)
    }
}
