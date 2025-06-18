mod cli;
mod commands;
mod config;
mod entries;
mod helpers;
mod log;
mod model;
mod plan;
// mod validation;

fn main() -> anyhow::Result<()> {
    cli::process()
}
