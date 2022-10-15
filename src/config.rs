use std::{fs::File, io::Read};
use serde_derive::{Deserialize, Serialize};

const CCAKE_CONFIG_FILE_NAME: &str = "ccake.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectMetadata {
    pub project_name: String,
    pub project_version: String,
    pub authors: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub metadata: ProjectMetadata,
}

pub fn read_config() -> Config {
    let mut file = File::open(CCAKE_CONFIG_FILE_NAME).expect("Failed to open ccake.toml file for reading!");

    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("Failed to read contents of ccake.toml file!");

    let config: Config = toml::from_str(&file_content).expect("Failed to deserialize content from ccake.toml file!");

    // TODO: Remove these println macro calls.
    println!("Project Name: {}", config.metadata.project_name);
    println!("Project Version: {}", config.metadata.project_version);
    println!("Project Author: {:?}", config.metadata.authors);

    return config;
}

pub fn write_config(config: &Config) {
    let config_as_str = toml::to_string(&config).expect("Failed to serialize config to string!");
    std::fs::write(CCAKE_CONFIG_FILE_NAME, config_as_str).expect("Failed to write to ccake.toml file!");
}