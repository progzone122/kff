use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use git2::Repository;
use serde::Deserialize;

use crate::config::TEMPLATES_DIR;

#[derive(Deserialize, Debug)]
pub struct Repo {
    name: String,
    url: String,
}

pub fn search(template_name: &str) -> anyhow::Result<Repo> {
    let data = r#"
    [
        {
            "name": "gtk2",
            "url": "https://github.com/progzone122/kff-gtk2-template"
        }
    ]
    "#;

    let repos: Vec<Repo> = serde_json::from_str(data)?;

    for repo in repos {
        if repo.name == template_name {
            return Ok(repo);
        }
    }

    Err(anyhow::anyhow!("Template {template_name} not found in repository"))
}


pub fn download(repo: &Repo) -> anyhow::Result<()> {
    let destination_path = TEMPLATES_DIR.join(&repo.name);
    if destination_path.exists() {
        loop {
            println!("Template '{}' has been previously downloaded. Use it?", repo.name);
            print!("(y/n) > ");
            io::stdout().flush()?;

            let mut answer = String::new();
            io::stdin().read_line(&mut answer)?;
            let answer = answer.trim().to_lowercase();

            match answer.as_str() {
                "n" => {
                    std::fs::remove_dir_all(&destination_path)?;
                    println!("Old template removed");
                    break;
                }
                "y" => {
                    println!("Using existing template");
                    return Ok(());
                }
                _ => {
                    println!("Error! Enter 'y' or 'n'");
                }
            }
        }
    }

    Repository::clone(&repo.url, &destination_path)?;

    println!("[] Cloned '{}' into {:?}", repo.url, destination_path);

    Ok(())
}