use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    //TODO: change all path to pathbuf directly
    #[arg(short, long)]
    pub path: Option<String>,
    #[arg(long)]
    pub debug: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(short, long)]
        path: Option<String>,
    },
    #[command(arg_required_else_help = true)]
    Render {
        #[arg(value_delimiter = ',')]
        comp_name: Vec<String>,
        #[arg(short, long)]
        path: Option<String>,
        #[arg(short = 'n', long)]
        dry_run: bool,
        #[arg(short = 'v', long)]
        verbose: bool,
        #[arg(long)]
        tree: bool,
        #[arg(long, value_delimiter = ',')]
        variants: Vec<String>,
    },
    #[command(arg_required_else_help = true)]
    Draw {
        #[arg(value_delimiter = ',')]
        sketch_names: Vec<String>,
        #[arg(short, long)]
        path: Option<String>,
        #[arg(short = 'n', long)]
        dry_run: bool,
        #[arg(short = 'v', long)]
        verbose: bool,
        #[arg(long)]
        tree: bool,
        #[arg(long, value_delimiter = ',')]
        variants: Vec<String>,
    },
    Schema,
}
