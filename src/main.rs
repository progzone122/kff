use std::process;
use clap::Parser;
use crate::cli::CliArgs;

mod cli;
mod template;
mod repository;

fn main() -> anyhow::Result<()> {
    let args = cli::CliArgs::parse();

    match args.command {
        cli::Commands::Generate(generate_args) => {
            println!("Searching for the {} template in the kff global repository...", generate_args.name);
            match repository::search(&generate_args.name) {
                Ok(repo) => {
                    println!("Template found, cloning...");
                    if let Err(e) = repository::download(&repo) {
                        eprintln!("[] ERROR: {e}");
                        process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("ERROR: {e}");
                    process::exit(1);
                }
            }
        }
    }
    
    Ok(())
}