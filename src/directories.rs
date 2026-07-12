use std::path::PathBuf;

pub fn dir() -> std::io::Result<PathBuf> {
    let data_dir = dirs::data_local_dir()
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Local data directory not found",
        ))?;

    let launcher_dir = data_dir.join(".OliviaLauncher");
    std::fs::create_dir_all(&launcher_dir)?;

    Ok(launcher_dir)
}

pub fn config_file() -> std::io::Result<PathBuf> {
    let cfg = dir()?.join("username.txt");

    if !cfg.exists() {
        std::fs::File::create(&cfg)?;
    }

    Ok(cfg)
}