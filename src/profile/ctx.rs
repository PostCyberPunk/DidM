use super::Backuper;
use super::walk::WalkerContext;
use crate::commands::{CommandsContext, CommandsRunner};
use crate::log::Logger;
use crate::model::profile::{Mode, Unit};
use crate::model::{Behaviour, Profile};
use crate::path::PathBufExtension;
use crate::plan::{PlanArgs, PlanContext};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub struct ProfileContext<'a> {
    pub name: &'a str,
    pub idx: usize,
    pub profile: &'a Profile,
    pub base_path: &'a PathBuf,
    pub behaviour: &'a Behaviour,
    pub backuper: &'a mut Backuper,
    pub args: &'a PlanArgs,
    pub logger: &'a Logger,
}

impl<'a> ProfileContext<'a> {
    pub fn new(
        name: &'a str,
        idx: usize,
        profile: &'a Profile,
        plan: &'a PlanContext,
        behaviour: &'a Behaviour,
        backuper: &'a mut Backuper,
    ) -> Self {
        let args = plan.args;
        let logger = plan.logger;
        //TODO: use vec[path] to avoid get path from configs
        let base_path = &plan.configs[idx].base_path;
        Self {
            name,
            idx,
            profile,
            base_path,
            behaviour,
            backuper,
            args,
            logger,
        }
    }

    pub fn apply(&mut self) -> Result<()> {
        let logger = self.logger;
        let profile = self.profile;
        let backuper = &mut self.backuper;

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

        if self.behaviour.should_backup() && self.profile.mode == Mode::Copy {
            let prefix = format!("profile_{}", self.name);
            backuper.set_ctx(prefix);
        }

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
        //Genrate target entries and backup them
        let entries: Vec<Option<(PathBuf, PathBuf)>> = entries
            .into_iter()
            .map(|entry| {
                let relative_path = match entry.strip_prefix(&source_root) {
                    Ok(p) => p,
                    Err(e) => {
                        logger.warn(&format!("Invalid entry path: {}", e));
                        return None;
                    }
                };
                let p = target_root.clone().join(relative_path);
                match backuper.backup(&p, relative_path, logger, || p.exists()) {
                    Ok(_) => Some((entry, p)),
                    Err(err) => {
                        logger.warn(&format!(
                            "Backup failed Skipping {},\n {}",
                            p.display(),
                            err
                        ));
                        None
                    }
                }
            })
            .collect();

        // TODO: apply entries

        //  TODO: empty_files、null_files、extra_rules

        cmds_runner.run_post_commands()?;

        backuper.drop_ctx();
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
