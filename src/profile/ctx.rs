use super::walk::WalkerContext;
use crate::commands::{CommandsContext, CommandsRunner};
use crate::log::Logger;
use crate::model::profile::{Mode, Unit};
use crate::model::{Behaviour, Profile};
use crate::path::PathBufExtension;
use crate::plan::PlanArgs;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub struct ProfileContext<'a> {
    pub name: &'a str,
    pub idx: usize,
    pub profile: &'a Profile,
    pub base_path: &'a PathBuf,
    pub behaviour: &'a Behaviour,
    pub args: &'a PlanArgs,
    pub logger: &'a Logger,
}

impl<'a> ProfileContext<'a> {
    pub fn new(
        name: &'a str,
        idx: usize,
        profile: &'a Profile,
        base_path: &'a PathBuf,
        behaviour: &'a Behaviour,
        args: &'a PlanArgs,
        logger: &'a Logger,
    ) -> Self {
        Self {
            name,
            idx,
            profile,
            base_path,
            behaviour,
            args,
            logger,
        }
    }

    pub fn apply(&self) -> Result<()> {
        let logger = self.logger;
        let profile = self.profile;

        let source_root = self
            .base_path
            .join(&profile.source_path)
            .canonicalize()
            .with_context(|| format!("Invalid source_path: {}", profile.source_path))?;
        let target_root = PathBuf::from(&profile.target_path)
            .resolve()?
            .canonicalize()
            .with_context(|| format!("Invalid target_path: {}", profile.target_path))?;
        logger.info(&format!("Source path: {}", source_root.display(),));
        logger.info(&format!("Target path: {}", target_root.display()));

        let commands_path = self
            .base_path
            .resolve_or_from(&self.profile.commands_path)?;
        let cmds_runner = CommandsRunner::new(
            CommandsContext {
                environment: &profile.environment,
                path: &commands_path,
                logger,
                args: self.args,
                stop_at_commands_error: self.behaviour.stop_at_commands_error.unwrap_or(false),
            },
            &self.profile.pre_build_commands,
            &self.profile.post_build_commands,
        );
        cmds_runner.run_pre_commands()?;

        logger.info("Generating entries ...");
        //Generate entries by walker
        let entries = WalkerContext::new(profile, &source_root, logger)
            .get_walker()?
            .run()?;
        // TODO: apply entries
        entries.iter().for_each(|entry| {
            logger.debug(&format!("Entry: {}", entry.to_string()));
        });
        //  TODO: empty_files、null_files、extra_rules

        cmds_runner.run_post_commands()?;

        Ok(())
    }
}

fn apply_entry(src: &Path, tgt: &Path, mode: &Mode, logger: &Logger) -> Result<()> {
    match mode {
        Mode::Symlink => {
            let _ = std::fs::remove_file(tgt);
            logger.info(&format!(
                "Symlinking {} -> {}",
                tgt.display(),
                src.display()
            ));
            std::os::unix::fs::symlink(src, tgt)
                .with_context(|| format!("symlink {:?} -> {:?}", src, tgt))?;
        }
        Mode::Copy => match src.is_dir() {
            true => {
                fs::create_dir_all(tgt)?;
                for entry in fs::read_dir(src)? {
                    let entry = entry?;
                    let sub_src = entry.path();
                    let sub_tgt = tgt.join(entry.file_name());
                    apply_entry(&sub_src, &sub_tgt, mode, logger)?;
                }
            }
            false => {
                logger.info(&format!("Copying {} -> {}", tgt.display(), src.display()));
                fs::copy(src, tgt)?;
            }
        },
    }
    Ok(())
}
