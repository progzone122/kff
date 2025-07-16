use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::env;
use std::env::temp_dir;

pub const ASCII_ART: &str = r#"
:::    :::  ::::::::::  :::::::::: 
:+:   :+:   :+:         :+:         
+:+  +:+    +:+         +:+           
+#++:++     :#::+::#    :#::+::#
+#+  +#+    +#+         +#+
#+#   #+#   #+#         #+#
###    ###  ###         ###       meow <3
"#;

pub static TEMPLATES_DIR: Lazy<PathBuf> = Lazy::new(|| {
    if let Ok(dir) = env::var("KFF_TEMPLATES_DIR") {
        PathBuf::from(dir)
    } else {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("~/.local/share"))
            .join("kff")
            .join("templates")
    }
});
pub static KSDK: Lazy<Option<String>> = Lazy::new(|| {
    env::var("KSDK").ok()
});

pub static HOME: Lazy<PathBuf> = Lazy::new(|| {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
});
pub static TEMP: Lazy<PathBuf> = Lazy::new(|| {
    temp_dir()
});
pub static REPOSITORY: &str = "https://raw.githubusercontent.com/progzone122/kff/main/templates.json";