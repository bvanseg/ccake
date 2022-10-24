use serde_derive::{Deserialize, Serialize};
use std::{fs::File, io::Read};

use crate::lib::terminal::ansi::warning;

/// Global properties for ccake project
/// For local properties, see Config
#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub default_c_compiler_dir: String,
    pub default_cpp_compiler_dir: String,
}
static CCAKE_SETTINGS_FILE_NAME: &str = "settings.toml";

impl Settings {
    pub fn read() -> Settings {
        let mut home_dir = dirs::home_dir().unwrap(); // TODO: Not safe.
        home_dir.push(".ccake");

        std::fs::create_dir_all(&home_dir)
            .expect("Failed to create directories to settings.toml path!");

        home_dir.push(CCAKE_SETTINGS_FILE_NAME);

        let default_settings = Settings {
            default_c_compiler_dir: "/path/to/c-compiler".to_string(),
            default_cpp_compiler_dir: "/path/to/cpp-compiler".to_string(),
        };

        let default_settings_str = toml::to_string(&default_settings)
            .expect("Failed to serialize default .ccake settings!");

        // If the .ccake settings file does not exist, create it and write the default data into it.
        // Otherwise, try to open the file and read into it.
        if !home_dir.exists() {
            // TODO: Make this constant somehow.
            std::fs::write(home_dir, default_settings_str)
                .expect("Failed to write to settings.toml file!");
            default_settings
        } else {
            match File::open(home_dir) {
                Ok(mut file) => {
                    let mut file_content = String::new();
                    let res = file.read_to_string(&mut file_content);

                    if res.is_err() {
                        warning("Failed to read text data from settings.toml, falling back on default settings.");
                        file_content = default_settings_str;
                    }

                    toml::from_str(&file_content).unwrap_or(default_settings)
                }
                Err(_) => {
                    warning("Failed to open settings.toml file for reading, falling back on default settings.");
                    default_settings
                }
            }
        }
    }
}

impl Settings {
    pub fn write(&self) {
        let settings_as_str =
            toml::to_string(&self).expect("Failed to serialize settings data to string!");

        let mut home_dir = dirs::home_dir().unwrap(); // TODO: Not safe.
        home_dir.push(".ccake");

        std::fs::create_dir_all(&home_dir)
            .expect("Failed to create directories to settings.toml path!");

        home_dir.push(CCAKE_SETTINGS_FILE_NAME);

        std::fs::write(home_dir, settings_as_str).expect("Failed to write to settings.toml file!");
    }
}
