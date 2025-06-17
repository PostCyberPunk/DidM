use super::Backuper;
use super::entry::Entries;
use super::walk::WalkerContext;
use crate::commands::{CommandsContext, CommandsRunner};
use crate::helpers::{Helpers, ResolvedPath};
use crate::log::Logger;
use crate::model::{Behaviour, Profile};
use crate::plan::{PlanArgs, PlanContext};
use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct ProfileContext<'a> {
    pub name: &'a str,
    pub profile: &'a Profile,
    pub base_path: &'a ResolvedPath,
    pub behaviour: &'a Behaviour,
    pub args: &'a PlanArgs,
    pub logger: &'a Logger,
    pub helpers: &'a Helpers,
}

impl<'a> ProfileContext<'a> {
    pub fn new(
        name: &'a str,
        base_path: &'a ResolvedPath,
        profile: &'a Profile,
        plan_ctx: &'a PlanContext,
        behaviour: &'a Behaviour,
        //FIX: make this imutable!! just initialize it with check_config
    ) -> Self {
        let args = plan_ctx.args;
        let logger = plan_ctx.logger;
        //TODO: use vec[path] to avoid get path from configs
        Self {
            name,
            profile,
            base_path,
            behaviour,
            args,
            logger,
            helpers: plan_ctx.helpers,
        }
    }

    //FIX: path iniialize refcator to new
    pub fn apply(&mut self) -> Result<()> {
        let logger = self.logger;
        let profile = self.profile;
        let behaviour = self.behaviour;
        let checker = &self.helpers.checker;
        let path_resolver = &self.helpers.path_resolver;

        //TODO: create a fucntion for this
        //1.the path should be resolved first
        //2.then we do a symlink check
        //3. ask user whether to readlink or cancel the action
        //NOTE:So... we should make a new resolved class...then path is now a mess
        let source_root = path_resolver
            .resolve_from(self.base_path, &profile.source_path)
            .with_context(|| format!("Invalid source_path: {}", profile.source_path))?;
        //TODO: we also need to check the source path
        let target_root = path_resolver
            .resolve_from(self.base_path, &profile.target_path)
            .with_context(|| format!("Invalid target_path: {}", profile.target_path))?;
        //FIX:this will never work since we are using canonicalize
        checker.target_exisit_or_create(target_root.get())?;
        logger.info(&format!("Source path: {}", source_root.di_string(),));
        logger.info(&format!("Target path: {}", target_root.di_string()));

        // if behaviour.should_backup() {
        //     let prefix = format!("profile_{}", self.name);
        // }

        let commands_path =
            path_resolver.resolve_from_or_base(self.base_path, &self.profile.commands_path)?;
        let cmds_runner = CommandsRunner::new(
            CommandsContext {
                environment: &profile.environment,
                path: commands_path.get(),
                logger,
                args: self.args,
                stop_at_commands_error: behaviour.stop_at_commands_error.unwrap_or(false),
            },
            &self.profile.pre_build_commands,
            &self.profile.post_build_commands,
        );
        cmds_runner.run_pre_commands()?;

        logger.info("Generating entries ...");
        //Generate entries by walker
        let entries = WalkerContext::new(profile, source_root.get(), logger)
            .get_walker()?
            .run()?;
        //Genrate target entries and backup them
        let entries: Vec<Option<(PathBuf, PathBuf)>> = entries
            .into_iter()
            .map(|entry| {
                let relative_path = match entry.strip_prefix(source_root.get()) {
                    Ok(p) => p,
                    Err(e) => {
                        logger.warn(&format!("Invalid entry path: {}", e));
                        return None;
                    }
                };
                let p = target_root.get().join(relative_path);
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

        let mut entries = Entries::new(
            entries,
            self.behaviour,
            logger,
            &self.profile.mode,
            self.args.is_dry_run,
        );
        entries.apple_entries()?;

        //  TODO: empty_files、null_files、additional_entries

        cmds_runner.run_post_commands()?;

        // backuper.drop_ctx(logger);
        Ok(())
    }
}
