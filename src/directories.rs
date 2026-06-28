/*extern crate directories;

use std::path::PathBuf;
use directories::BaseDirs;
pub fn dir() -> PathBuf {
    if let Some(launcher_dir) = BaseDirs::new() {
        launcher_dir.data_local_dir().to_path_buf();

    }
}*/

extern crate dirs;

use std::path::PathBuf;
pub fn dir() -> Option<PathBuf> {
    let mut data_dir = dirs::data_local_dir()?;
    let launcher_dir = data_dir.join(".OliviaLauncher");
    std::fs::create_dir_all(&launcher_dir).expect("Failed to create launcher dir");
    Some(launcher_dir)
}