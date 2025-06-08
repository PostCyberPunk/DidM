use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    //TODO: change all path to pathbuf directly
    #[arg(short, long)]
    pub path: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(short, long)]
        path: Option<String>,
    },
    #[command(arg_required_else_help = true)]
    Deploy {
        #[arg(short, long)]
        path: Option<String>,
        plan: String,
        #[arg(short = 'n', long)]
        dry_run: bool,
        #[arg(short = 'v', long)]
        verbose: bool,
        //TODO: a preview tree will be nice
    },
}
