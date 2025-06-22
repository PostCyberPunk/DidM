mod bakcup;
mod cli;
mod commands;
mod composition;
mod config;
mod entries;
mod model;
mod utils;
// mod validation;

fn main() -> anyhow::Result<()> {
    cli::process()
}
