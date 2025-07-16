use clap::{Args, Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Generate(GenerateArgs),
    Doctor,
    Install(InstallerArgs)
}

#[derive(Args, Debug)]
pub struct GenerateArgs {
    pub(crate) name: String,
}
#[derive(Args, Debug)]
pub struct InstallerArgs {
    pub(crate) names: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    #[command(subcommand)]
    pub(crate) command: Commands
}