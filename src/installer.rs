use std::fmt::format;
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;
use reqwest::blocking::Client;
use serde::Deserialize;
use anyhow::{anyhow, Result};

pub fn toolchain(target: &str) -> Result<()> {
    let repo: &str = "koreader/koxtoolchain";
    let version: &str = "latest";
    let asset_name: &str = &format!("{target}.tar.gz");

    let url = get_release_download_url(repo, version, asset_name)?;
    println!("Downloading from: {}", url);

    download_and_extract(&url, ".")?;

    Ok(())
}

pub fn sdk(target: &str) -> Result<()> {
    let repo: &str = "koreader/koxtoolchain";
    let version: &str = "latest";
    let asset_name: &str = &format!("{target}.tar.gz");

    let url = get_release_download_url(repo, version, asset_name)?;
    println!("Downloading from: {}", url);

    download_and_extract(&url, ".")?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct ReleaseAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<ReleaseAsset>,
}

fn get_release_download_url(repo: &str, tag: &str, target_filename: &str) -> Result<String> {
    let url = if tag == "latest" {
        format!("https://api.github.com/repos/{}/releases/latest", repo)
    } else {
        format!("https://api.github.com/repos/{}/releases/tags/{}", repo, tag)
    };

    let client = Client::builder().user_agent("kff").build()?;
    let release: Release = client.get(&url).send()?.json()?;

    for asset in release.assets {
        if asset.name.ends_with(target_filename) {
            return Ok(asset.browser_download_url);
        }
    }

    Err(anyhow!("No asset named '{}' found in release '{}'", target_filename, tag).into())
}

fn download_and_extract(url: &str, out_dir: &str) -> Result<()> {
    let response = reqwest::blocking::get(url)?;
    if !response.status().is_success() {
        return Err(anyhow!("Download failed: {}", response.status()).into());
    }

    let tar_gz = GzDecoder::new(BufReader::new(Cursor::new(response.bytes()?)));
    let mut archive = Archive::new(tar_gz);
    archive.unpack(out_dir)?;
    Ok(())
}