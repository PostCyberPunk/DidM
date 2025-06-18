use anyhow::Result;

use crate::log::Logger;

use super::CommandsContext;

pub struct CommandsRunner<'a> {
    pub context: Vec<CommandsContext<'a>>,
    pub logger: &'a Logger,
    pub is_dryrun: bool,
}

impl<'a> CommandsRunner<'a> {
    pub fn new(logger: &'a Logger, is_dryrun: bool) -> Self {
        Self {
            context: Vec::new(),
            logger,
            is_dryrun,
        }
    }

    pub fn add_context(&mut self, context: CommandsContext<'a>) {
        self.context.push(context);
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

        self.logger.info(&format!("Running {} commands", label));

        for ctx in &self.context {
            ctx.run(commands_selector(ctx), self.logger, self.is_dryrun)?;
        }

        Ok(())
    }
}
