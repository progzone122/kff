use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::env;
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