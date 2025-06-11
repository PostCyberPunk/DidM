mod cli;
mod commands;
mod config;
mod log;
mod model;
mod path;
mod plan;
mod profile;

fn main() -> anyhow::Result<()> {
    cli::process()
}
