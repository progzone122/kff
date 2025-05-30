use std::io::{BufRead, Read};
use std::io::{BufReader, Cursor};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::Instant;
use flate2::read::GzDecoder;
use tar::Archive;
use reqwest::blocking::Client;
use serde::Deserialize;
use anyhow::{anyhow, Context, Result};
use fs_extra::dir;
use git2::{FetchOptions, Progress, RemoteCallbacks, Repository, SubmoduleUpdateOptions};
use indicatif::{ProgressBar, ProgressStyle};
use crate::config::{HOME, TEMP, TEMPLATES_DIR};


fn run_command(cmd: &mut Command, desc: &str) -> Result<()> {
    println!("Running {desc}...");

    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("failed to start {desc}"))?;

    let stdout = child.stdout.take().expect("no stdout");
    let stderr = child.stderr.take().expect("no stderr");

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    // Параллельно читаем stdout и stderr
    let stdout_thread = std::thread::spawn(move || {
        for line in stdout_reader.lines().flatten() {
            println!("{}", line);
        }
    });

    let stderr_thread = std::thread::spawn(move || {
        for line in stderr_reader.lines().flatten() {
            eprintln!("{}", line);
        }
    });

    let status = child
        .wait()
        .with_context(|| format!("failed to wait for {desc}"))?;

    let _ = stdout_thread.join();
    let _ = stderr_thread.join();

    if !status.success() {
        return Err(anyhow!("{desc} failed with exit code {:?}", status.code()));
    }

    Ok(())
}

pub fn clone_with_progress<P: AsRef<Path>>(url: &str, dest: P) -> Result<Repository> {
    println!("Cloning repository: {}", url);
    println!("Destination: {:?}", dest.as_ref());

    let pb = Arc::new(ProgressBar::new(0));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} objects ({eta})")?
            .progress_chars("=>-"),
    );
    pb.set_message("Cloning");

    let pb_clone = Arc::clone(&pb);

    // Настраиваем коллбек для прогресса
    let mut callbacks = RemoteCallbacks::new();
    callbacks.transfer_progress(move |stats: Progress| {
        let total = stats.total_objects();
        let received = stats.received_objects();
        if total > 0 {
            pb_clone.set_length(total as u64);
            pb_clone.set_position(received as u64);
        }
        true
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);

    let start = Instant::now();

    // Клонируем основной репозиторий
    let repo = builder.clone(url, dest.as_ref())
        .with_context(|| format!("Failed to clone {}", url))?;

    pb.finish_with_message("✅ Clone complete");
    println!("Cloned in {:?}", start.elapsed());

    println!("Updating submodules with progress...");

    // Для подмодулей нам нужен отдельный прогресс-бар
    let pb_sub = Arc::new(ProgressBar::new(0));
    pb_sub.set_style(
        ProgressStyle::default_bar()
            .template("Submodules: {msg} [{bar:40.green/black}] {pos}/{len} ({eta})")?
            .progress_chars("=>-"),
    );
    pb_sub.set_message("Updating");

    let pb_sub_clone = Arc::clone(&pb_sub);

    // Коллбек для подмодулей
    let mut submodule_callbacks = RemoteCallbacks::new();
    submodule_callbacks.transfer_progress(move |stats: Progress| {
        let total = stats.total_objects();
        let received = stats.received_objects();
        if total > 0 {
            pb_sub_clone.set_length(total as u64);
            pb_sub_clone.set_position(received as u64);
        }
        true
    });

    let mut sub_fetch_options = FetchOptions::new();
    sub_fetch_options.remote_callbacks(submodule_callbacks);

    let mut submodule_update_opts = SubmoduleUpdateOptions::new();
    submodule_update_opts.fetch(sub_fetch_options);

    // Обновляем каждый подмодуль
    for mut sub in repo.submodules()? {
        sub.update(true, Some(&mut submodule_update_opts))
            .with_context(|| format!("Failed to update submodule {}", sub.name().unwrap_or("<unknown>")))?;
    }

    pb_sub.finish_with_message("✅ Submodules updated");

    Ok(repo)
}


pub fn toolchain(target: &str) -> Result<()> {
    let repo: &str = "koreader/koxtoolchain";
    let version: &str = "latest";
    let asset_name: &str = &format!("{target}.tar.gz");

    let url = get_release_download_url(repo, version, asset_name)?;
    println!("Downloading from: {}", url);

    let path_string: String = HOME.to_string_lossy().into_owned();
    download_and_extract(&url, &path_string)?;

    Ok(())
}

pub fn sdk(target: &str) -> Result<()> {
    let destination_path = TEMP.join("kindle-sdk");
    let url: &str = "https://github.com/KindleModding/kindle-sdk.git";

    println!("Downloading SDK...");

    dir::create(&destination_path, true)?;

    clone_with_progress(&url, &destination_path)?;

    println!("Cloned 'kindle-sdk' into {:?}", destination_path);

    let script_path = destination_path.join("gen-sdk.sh");

    run_command(Command::new("chmod").arg("+x").arg(&script_path), "chmod gen-sdk.sh")?;
    run_command(Command::new("sh").arg(&script_path).arg(target), "gen-sdk.sh")?;

    println!("SDK successfully installed. It's time to forge!");

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

    let total_size = response
        .content_length()
        .ok_or_else(|| anyhow!("Failed to get content length"))?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("##-"),
    );
    pb.set_message("Downloading");

    let mut downloaded: u64 = 0;
    let mut content = Vec::with_capacity(total_size as usize);
    let mut reader = response.take(total_size);

    let mut buffer = [0; 8192];
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        content.extend_from_slice(&buffer[..n]);
        downloaded += n as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete, extracting...");

    let tar_gz = GzDecoder::new(BufReader::new(Cursor::new(content)));
    let mut archive = Archive::new(tar_gz);
    archive.unpack(out_dir)?;

    println!("Extraction finished.");
    Ok(())
}