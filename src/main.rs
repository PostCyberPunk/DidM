mod cli;
mod commands;
mod config;
mod entries;
mod log;
mod model;
mod plan;
mod utils;
// mod validation;

fn main() -> anyhow::Result<()> {
    cli::process()
}
