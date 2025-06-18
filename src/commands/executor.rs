use std::{
    collections::HashMap,
    io,
    path::Path,
    process::{Command, Output},
};

pub struct CommandExecutor<'a> {
    pub environment: &'a HashMap<String, String>,
    pub path: &'a Path,
    pub command: &'a String,
}

impl<'a> CommandExecutor<'a> {
    pub fn run(&self) -> io::Result<Output> {
        Command::new("sh")
            .envs(self.environment)
            .current_dir(&self.path)
            .arg("-c")
            .arg(self.command)
            .output()
    }
}
