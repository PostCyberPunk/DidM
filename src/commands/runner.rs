use anyhow::Result;
use tracing::info;

use crate::{
    model::Sketch,
    utils::{PathResolver, ResolvedPath},
};

use super::CommandsContext;

pub struct CommandsRunner<'a> {
    context: Vec<CommandsContext<'a>>,
    is_dryrun: bool,
}

impl<'a> CommandsRunner<'a> {
    pub fn new(is_dryrun: bool) -> Self {
        Self {
            context: Vec::new(),
            is_dryrun,
        }
    }

    pub fn add_context(&mut self, context: CommandsContext<'a>) {
        self.context.push(context);
    }
    pub fn add_sketch_context(
        &mut self,
        sketch: &'a Sketch,
        base_path: &ResolvedPath,
        stop_at_commands_error: bool,
    ) -> Result<()> {
        let commands_path =
            PathResolver::resolve_from_or_base(base_path, &sketch.commands_path)?.into_pathbuf();
        self.context.push(CommandsContext::new(
            &sketch.environment,
            commands_path,
            stop_at_commands_error,
            &sketch.pre_build_commands,
            &sketch.post_build_commands,
        ));
        Ok(())
    }

    pub fn run_pre_commands(&self) -> Result<()> {
        self.run_commands("pre", |ctx| ctx.pre_commands)
    }

    pub fn run_post_commands(&self) -> Result<()> {
        self.run_commands("post", |ctx| ctx.post_commands)
    }

    pub(crate) fn run_commands<F>(&self, label: &str, commands_selector: F) -> Result<()>
    where
        F: Fn(&CommandsContext<'a>) -> &'a [String],
    {
        if self.context.is_empty() {
            return Ok(());
        }

        info!("Running {} commands", label);

        for ctx in &self.context {
            ctx.run(commands_selector(ctx), self.is_dryrun)?;
        }

        Ok(())
    }
}
