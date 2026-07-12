
use crate::{directories, MainWindow};

extern crate dirs;  //xz
use std::fs;
use slint::platform::Key::Return;

pub fn load_username() -> String {
    let username_path = directories::config_file().expect("Failed to read config file");
    let text = fs::read_to_string(&username_path).expect("failed to read username");
    text
}

pub fn save_username(username: slint::SharedString) -> std::io::Result<()> {
    let username_path = directories::config_file()?;
    fs::write(username_path, username)?;
    Ok(())
}
