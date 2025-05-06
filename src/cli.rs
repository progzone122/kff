use clap::{Args, Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Commands {
    New(NewArgs)
}

#[derive(Args, Debug)]
struct NewArgs {
    name: String,

    #[arg(long)]
    gtk: bool,
    
    
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    #[command(subcommand)]
    command: Commands
}