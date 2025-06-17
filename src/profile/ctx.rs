use super::Backuper;
use super::entry::Entries;
use super::walk::WalkerContext;
use crate::commands::{CommandsContext, CommandsRunner};
use crate::helpers::{self, Helpers, ResolvedPath};
use crate::log::Logger;
use crate::model::{Behaviour, Profile};
use crate::plan::PlanArgs;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct ProfileContext<'a> {
    pub args: &'a PlanArgs,
    pub logger: &'a Logger,
    pub helpers: &'a Helpers,

    pub name: &'a str,
    pub profile: &'a Profile,
    pub base_path: &'a ResolvedPath,
    pub behaviour: Behaviour,
    commands_runner: CommandsRunner<'a>,
}

impl<'a> ProfileContext<'a> {
    pub fn new(
        args: &'a PlanArgs,
        logger: &'a Logger,
        helpers: &'a Helpers,
        name: &'a str,
        base_path: &'a ResolvedPath,
        profile: &'a Profile,
        behaviour: Behaviour,
    ) -> Result<Self> {
        //PreparePath
        //TODO: create a fucntion for this
        //1.the path should be resolved first
        //2.then we do a symlink check
        //3. ask user whether to readlink or cancel the action
        //NOTE:So... we should make a new resolved class...then path is now a mess
        let path_resolver = &helpers.path_resolver;
        let source_root = path_resolver
            .resolve_from(base_path, &profile.source_path)
            .with_context(|| format!("Invalid source_path: {}", profile.source_path))?;
        //TODO: we also need to check the source path
        let target_root = path_resolver
            .resolve_from(base_path, &profile.target_path)
            .with_context(|| format!("Invalid target_path: {}", profile.target_path))?;
        //checkPath
        //FIX:this will never work since we are using canonicalize
        helpers.checker.target_exisit_or_create(target_root.get())?;
        logger.info(&format!("Source path: {}", source_root.di_string(),));
        logger.info(&format!("Target path: {}", target_root.di_string()));

        //Prepare Behaviour
        let behaviour = behaviour.override_by(&profile.override_behaviour);

        //Prepare Commands

        // if behaviour.should_backup() {
        //     let prefix = format!("profile_{}", self.name);
        // }

        let commands_path =
            path_resolver.resolve_from_or_base(base_path, &profile.commands_path)?;
        let commands_runner = CommandsRunner::new(
            CommandsContext {
                environment: &profile.environment,
                path: commands_path.into_pathbuf(),
                logger,
                args,
                stop_at_commands_error: behaviour.stop_at_commands_error.unwrap_or(false),
            },
            &profile.pre_build_commands,
            &profile.post_build_commands,
        );
        //Generate entries by walker
        logger.info("Generating entries ...");
        // let mut entries = Entries::new(
        //     entries,
        //     &self.behaviour,
        //     logger,
        //     &self.profile.mode,
        //     self.args.is_dry_run,
        // );
        Ok(Self {
            name,
            profile,
            base_path,
            behaviour,
            args,
            logger,
            helpers,
            commands_runner,
        })
    }

    pub fn apply(&self) -> Result<()> {
        self.commands_runner.run_pre_commands()?;

        // entries.apple_entries()?;

        //  TODO: empty_files、null_files、additional_entries

        self.commands_runner.run_post_commands()?;

        // backuper.drop_ctx(logger);
        Ok(())
    }
}
