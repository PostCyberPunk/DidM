use crate::log::Logger;
use crate::model::sketch::{Mode, Sketch, Unit};
use anyhow::{Context, Result};
use ignore::WalkBuilder;
use ignore::overrides::OverrideBuilder;
use std::path::{Path, PathBuf};

pub struct WalkerContext<'a> {
    pub base_path: &'a Path,
    pub unit: &'a Unit,
    pub mode: &'a Mode,
    pub ignore: &'a Vec<String>,
    pub respect_gitignore: &'a bool,
    pub ignore_hidden: &'a bool,
    pub logger: &'a Logger,
    walker: Option<WalkBuilder>,
}
impl<'a> WalkerContext<'a> {
    pub fn new(profile: &'a Sketch, base_path: &'a Path, logger: &'a Logger) -> Self {
        Self {
            base_path,
            unit: &profile.unit,
            mode: &profile.mode,
            ignore: &profile.ignore,
            respect_gitignore: &profile.respect_gitignore,
            ignore_hidden: &profile.ignore_hidden,
            logger,
            walker: None,
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
        overrides.add("!.didm_backup")?;

        for ignore_item in self.ignore {
            overrides
                .add(&format!("!{}", ignore_item))
                .context(format!("Failed to add ignore item:{}", ignore_item))?;
        }
        if *self.unit == Unit::Dir && *self.mode == Mode::Symlink {
            walker.max_depth(Some(1));
        };
        walker
            .overrides(overrides.build()?)
            .hidden(*self.ignore_hidden)
            .git_ignore(*self.respect_gitignore);

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
        if *self.unit == Unit::Dir && *self.mode == Mode::Symlink {
            entries.remove(0);
        }

        Ok(entries)
    }
}
