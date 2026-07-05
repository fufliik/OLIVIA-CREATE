extern crate dirs;

use std::path::PathBuf;
pub fn dir() -> Option<PathBuf> {
    let data_dir = dirs::data_local_dir()?;
    let launcher_dir = data_dir.join(".OliviaLauncher");
    std::fs::create_dir_all(&launcher_dir).expect("Failed to create launcher dir");
    Some(launcher_dir)
}
pub fn config_file() -> Option<PathBuf> {
    let cfg = dir()?.join("cfg.json");
    if !cfg.exists() {
        std::fs::File::create(&cfg).expect("Не удалось создать файл");
    }
    Some(cfg)
}
