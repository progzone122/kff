use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use git2::Repository;
use serde::Deserialize;

use crate::config::{REPOSITORY, TEMPLATES_DIR};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RepoSource {
    Local,
    Remote(String), // git url
}

#[derive(Deserialize, Debug)]
pub struct Repo {
    pub(crate) name: String,
    pub(crate) url: RepoSource,
}
impl Repo {
    pub fn new(name: &str, url: RepoSource) -> Self {
        Self {
            name: name.to_string(),
            url,
        }
    }
}

pub fn choose_local_or_download(repo: &Repo) -> anyhow::Result<bool> {
    if let RepoSource::Local = repo.url {
        loop {
            println!("Local template '{}' found. Use it? (y/n)", repo.name);
            print!("> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_lowercase();

            match input.as_str() {
                "y" | "yes" => return Ok(true),
                "n" | "no" => return Ok(false),
                _ => println!("Please enter 'y' or 'n'."),
            }
        }
    }

    Ok(false)
}

pub fn search(template_name: &str) -> anyhow::Result<Repo> {
    let response = reqwest::blocking::get(REPOSITORY)?;
    let data = response.text()?;

    let repos: Vec<Repo> = serde_json::from_str(&data)?;

    for repo in repos {
        if repo.name == template_name {
            return Ok(repo);
        }
    }

    Err(anyhow::anyhow!("Template {template_name} not found in repository"))
}


pub fn download(repo: &Repo) -> anyhow::Result<()> {
    let destination_path = TEMPLATES_DIR.join(&repo.name);

    let RepoSource::Remote(url) = &repo.url else {
        return Err(anyhow::anyhow!(
            "download() called on a local template '{}'",
            repo.name
        ));
    };

    if destination_path.exists() {
        std::fs::remove_dir_all(&destination_path)?;
    }

    Repository::clone(url, &destination_path)?;
    println!("Cloned '{}' into {:?}", url, destination_path);
    Ok(())
}