mod cli;
mod config;
mod model;
mod path;

fn main() -> anyhow::Result<()> {
    cli::process()
}
