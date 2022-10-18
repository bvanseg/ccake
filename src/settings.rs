use dirs;
use serde_derive::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub default_compiler_dir: String,
}

static CCAKE_SETTINGS_FILE_NAME: &str = "settings.toml";

pub fn read_settings() -> Settings {
    let mut home_dir = dirs::home_dir().unwrap(); // TODO: Not safe.
    home_dir.push(".ccake");

    std::fs::create_dir_all(&home_dir)
        .expect("Failed to create directories to settings.toml path!");

    home_dir.push(CCAKE_SETTINGS_FILE_NAME);

    let mut file = File::open(home_dir).expect("Failed to open settings.toml file for reading!");

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Failed to read contents of settings.toml file!");

    return toml::from_str(&file_content)
        .expect("Failed to deserialize content from settings.toml file!");
}

pub fn write_settings(settings: &Settings) {
    let settings_as_str =
        toml::to_string(&settings).expect("Failed to serialize settings data to string!");

    let mut home_dir = dirs::home_dir().unwrap(); // TODO: Not safe.
    home_dir.push(".ccake");

    std::fs::create_dir_all(&home_dir)
        .expect("Failed to create directories to settings.toml path!");

    home_dir.push(CCAKE_SETTINGS_FILE_NAME);

    std::fs::write(home_dir, settings_as_str).expect("Failed to write to settings.toml file!");
}
