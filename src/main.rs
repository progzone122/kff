use std::collections::HashMap;
use std::env::{temp_dir};
use std::path::{PathBuf};
use std::{process};
use clap::Parser;
use fs_extra::dir::{copy, CopyOptions};
use fs_extra::{dir, file};
use crate::config::TEMPLATES_DIR;

mod cli;
mod template;
mod repository;
mod config;
mod doctor;
mod installer;

fn main() -> anyhow::Result<()> {
    let args = cli::CliArgs::parse();

    match args.command {
        cli::Commands::Generate(generate_args) => {
            println!("Kindle Fucking Forge generate started...");
            println!("{}", config::ASCII_ART);
            println!("Searching for the {} template in the kff global repository...", generate_args.name);
            match repository::search(&generate_args.name) {
                Ok(repo) => {
                    println!("Template found, cloning...");
                    if let Err(e) = repository::download(&repo) {
                        eprintln!("ERROR: {e}");
                        process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("ERROR: {e}");
                    process::exit(1);
                }
            }

            println!("Parsing the template.json file...");
            let tdir: PathBuf = TEMPLATES_DIR.join(&generate_args.name);

            // tmp dir
            let tmp_template_path = temp_dir().join("kff").join(&generate_args.name);
            if tmp_template_path.exists() {
                std::fs::remove_dir_all(&tmp_template_path)?;
            }

            // Copying a template to a tmp dir
            let mut options = CopyOptions::new();
            options.overwrite = true;
            options.copy_inside = true;
            copy(&tdir, &tmp_template_path, &options)?;

            match template::Template::parse_from_file(&tmp_template_path.join("template.json")) {
                Ok(repo) => {
                    println!("Starting the '{}' template generator", generate_args.name);
                    let answers: HashMap<String, String> = repo.ask_questions();
                    repo.apply_replacements(&answers, &tmp_template_path)?;

                    // Unnecessary files/dirs need to be deleted
                    file::remove(&tmp_template_path.join("template.json"))?;
                    dir::remove(&tmp_template_path.join(".git"))?;

                    let out_path = std::env::current_dir()?.join(&answers.get("app_name").unwrap_or(&generate_args.name));
                    if out_path.exists() {
                        std::fs::remove_dir_all(&out_path)?;
                    }

                    copy(&tmp_template_path, &out_path, &options)?;

                    println!("Project generated at: {}", out_path.display());
                }
                Err(e) => {
                    eprintln!("ERROR: {e}");
                    process::exit(1);
                }
            }
        }
        cli::Commands::Doctor => {
            doctor::run()?;
        }
        cli::Commands::Install(installer_args) => {
            match installer_args.names.get(0).map(String::as_str) {
                Some("toolchain") => {
                    if let Some(target) = installer_args.names.get(1) {
                        installer::toolchain(target)?;
                    } else {
                        eprintln!("[ERROR] Missing toolchain target. Example: `kff install toolchain kindlehf`");
                    }
                },
                Some("sdk") => {
                    if let Some(target) = installer_args.names.get(1) {
                        installer::sdk(target)?;
                    } else {
                        eprintln!("[ERROR] Missing sdk target. Example: `kff install sdk kindlehf`");
                    }
                }
                Some(unknown) => {
                    eprintln!("Unknown install target: {unknown}");
                }
                None => {
                    eprintln!("No install subcommand given");
                }
            }
        }
    }

    Ok(())
}