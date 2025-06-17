mod cli;
mod commands;
mod config;
mod helpers;
mod log;
mod model;
mod plan;
mod profile;
// mod validation;

fn main() -> anyhow::Result<()> {
    cli::process()
}
