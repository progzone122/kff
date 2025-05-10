use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::{anyhow, Result};

pub fn run() -> Result<()> {
    let ksdk = env::var("KSDK").ok();
    let lines = if let Some(ksdk_path) = ksdk.as_deref() {
        let file_path = format!("{}/meson-crosscompile.txt", ksdk_path);
        read_first_lines(&file_path).unwrap_or_else(|e| {
            eprintln!("Error reading file: {}", e);
            vec![]
        })
    } else {
        vec![]
    };

    print_ksdk(ksdk.as_deref(), lines);
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
pub fn print_ksdk(ksdk: Option<&str>, mesonfile: Vec<String>) {
    println!("--- KFF Doctor ---");
    println!(
        "KSDK env: {}\nKSDK: {}",
        if ksdk.is_some() { "[OK]" } else { "[NOT SET]" },
        ksdk.unwrap_or("[ERROR]")
    );
    print!("\nmeson-crosscompile.txt file: ");
    if !mesonfile.is_empty() {
        println!("");
        for line in mesonfile {
            println!("{}", line);
        }
        println!("...");
    } else {
        println!("[ERROR]");
    }
}