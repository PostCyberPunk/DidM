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
        let path_resolver = &helpers.path_resolver;
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
}
