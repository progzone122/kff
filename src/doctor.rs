use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::{anyhow, Result};
use glob::glob;
use crate::config::KSDK;

pub fn run() -> Result<()> {
    if let Some(ksdk_path) = KSDK.as_deref() {
        let pattern = format!("{}/arm-*-linux-gnueabihf/meson-crosscompile.txt", ksdk_path);
        let mut files_data = Vec::new();
        let mut found_any = false;

        for entry in glob(&pattern)? {
            match entry {
                Ok(path) => {
                    found_any = true;
                    let path_str = path.to_string_lossy().to_string();
                    match read_first_lines(&path_str) {
                        Ok(lines) => {
                            files_data.push((path_str, lines));
                        }
                        Err(e) => {
                            eprintln!("Error reading '{}': {}", path.display(), e);
                        }
                    }
                }
                Err(e) => eprintln!("Glob error: {}", e),
            }
        }

        if !found_any {
            println!("No files found matching pattern: {}", pattern);
        }

        print_ksdk(KSDK.as_deref(), files_data);
    } else {
        println!("KSDK env not set");
    }

    Ok(())
}

pub fn read_first_lines(ksdk: &str) -> Result<Vec<String>> {
    let path = Path::new(ksdk);

    let file = File::open(path).map_err(|err| anyhow!("File '{}' not found: {}", ksdk, err))?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines().take(5) {
        match line {
            Ok(line_content) => lines.push(line_content),
            Err(e) => return Err(anyhow!("Error reading from file '{}': {}", ksdk, e)),
        }
    }

    if lines.is_empty() {
        return Err(anyhow!("File '{}' is empty or damaged", ksdk));
    }

    Ok(lines)
}
pub fn print_ksdk(ksdk: Option<&str>, mesonfiles: Vec<(String, Vec<String>)>) {
    println!("--- KFF Doctor ---");
    println!(
        "KSDK env: {}\nKSDK: {}",
        if ksdk.is_some() { "[OK]" } else { "[NOT SET]" },
        ksdk.unwrap_or("[ERROR]")
    );

    if mesonfiles.is_empty() {
        println!("\nmeson-crosscompile.txt file(s) (SDK): [ERROR]");
    } else {
        println!("\nmeson-crosscompile.txt file(s) (SDK):");
        for (filename, lines) in mesonfiles {
            println!("Found file: {}", filename);
            for line in lines {
                println!("{}", line);
            }
            println!("---");
        }
    }
}