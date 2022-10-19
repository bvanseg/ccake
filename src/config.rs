use serde_derive::{Deserialize, Serialize};
use std::{fs::File, io::Read};

const CCAKE_CONFIG_FILE_NAME: &str = "ccake.toml";

#[derive(Debug, Deserialize, Serialize)]
pub enum ProjectType {
    Binary,
    Library,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectProperties {
    pub ccake_version: String,
    pub project_name: String,
    pub project_version: String,
    pub authors: Option<Vec<String>>,

    pub language: String,
    pub project_type: ProjectType,
    pub src_dir: Option<String>,
    pub out_dir: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompilerProperties {
    pub c_compiler_dir: String,
    pub compiler_args: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub project_properties: ProjectProperties,
    pub compiler_properties: CompilerProperties,
}

pub fn read_config() -> Config {
    let mut file =
        File::open(CCAKE_CONFIG_FILE_NAME).expect("Failed to open ccake.toml file for reading!");

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Failed to read contents of ccake.toml file!");

    return toml::from_str(&file_content)
        .expect("Failed to deserialize content from ccake.toml file!");
}

pub fn write_config(config: &Config, sub_path: &Option<&String>) {
    let config_as_str = toml::to_string(&config).expect("Failed to serialize config to string!");

    if let Some(path) = sub_path {
        let ccake_path = format!("{}{}{}", path, "/", CCAKE_CONFIG_FILE_NAME);
        std::fs::create_dir_all(path).expect("Failed to create directories to ccake.toml path!");
        std::fs::write(ccake_path, config_as_str).expect("Failed to write to ccake.toml file!");
        return;
    }

    std::fs::write(CCAKE_CONFIG_FILE_NAME, config_as_str)
        .expect("Failed to write to ccake.toml file!");
}
