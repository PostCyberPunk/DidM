mod cli;
mod commands;
mod config;
mod log;
mod model;
mod path;

fn main() -> anyhow::Result<()> {
    cli::process()
}
