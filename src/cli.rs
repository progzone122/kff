use clap::{Args, Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Generate(GenerateArgs)
}

#[derive(Args, Debug)]
pub struct GenerateArgs {
    pub(crate) name: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    #[command(subcommand)]
    pub(crate) command: Commands
}