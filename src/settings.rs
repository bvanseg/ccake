use dirs;
use serde_derive::{Deserialize, Serialize};
use std::{fs::File, io::Read};

use crate::terminal::ansi::warning;

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

    let default_settings = Settings {
        default_compiler_dir: "/path/to/compiler".to_string(),
    };

    let default_settings_str =
        toml::to_string(&default_settings).expect("Failed to serialize default .ccake settings!");

    // If the .ccake settings file does not exist, create it and write the default data into it.
    // Otherwise, try to open the file and read into it.
    if !home_dir.exists() {
        // TODO: Make this constant somehow.
        std::fs::write(home_dir, default_settings_str)
            .expect("Failed to write to settings.toml file!");
        return default_settings;
    } else {
        match File::open(home_dir) {
            Ok(mut file) => {
                let mut file_content = String::new();
                let res = file.read_to_string(&mut file_content);

                if let Err(_) = res {
                    warning("Failed to read text data from settings.toml, falling back on default settings.");
                    file_content = default_settings_str;
                }

                return toml::from_str(&file_content).unwrap_or(default_settings);
            }
            Err(_) => {
                warning("Failed to open settings.toml file for reading, falling back on default settings.");
                return default_settings;
            }
        };
    }
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
