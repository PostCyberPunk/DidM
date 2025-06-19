use std::{
    collections::HashMap,
    io,
    path::Path,
    process::{Command, Output},
};

pub struct CommandExecutor<'a> {
    environment: &'a HashMap<String, String>,
    path: &'a Path,
    command: &'a String,
}

impl<'a> CommandExecutor<'a> {
    pub fn new(
        environment: &'a HashMap<String, String>,
        path: &'a Path,
        command: &'a String,
    ) -> Self {
        Self {
            environment,
            path,
            command,
        }
    }
    pub fn run(&self) -> io::Result<Output> {
        Command::new("sh")
            .envs(self.environment)
            .current_dir(self.path)
            .arg("-c")
            .arg(self.command)
            .output()
    }
}
