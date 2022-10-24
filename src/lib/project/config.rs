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
    src_dir: Option<String>,
    out_dir: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompilerProperties {
    compiler_dir: Option<String>,
    compiler_args: Option<String>,
}

/// Local properties for ccake project
/// For global properties, see Settings
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub project_properties: ProjectProperties,
    pub compiler_properties: Option<CompilerProperties>,
}

impl Config {
    pub fn new(
        project_name: String,
        project_version: String,
        project_authors: String,
        ccake_version: String,
        project_language: String,
    ) -> Self {
        Config {
            project_properties: ProjectProperties {
                project_name,
                project_version,
                authors: Some(
                    project_authors
                        .trim()
                        .split(',')
                        .map(|f| f.trim().to_string())
                        .collect(),
                ),
                ccake_version,
                language: project_language,
                project_type: ProjectType::Binary,
                src_dir: None,
                out_dir: None,
            },
            compiler_properties: None,
        }
    }

    pub fn read() -> Self {
        let mut file = File::open(CCAKE_CONFIG_FILE_NAME)
            .expect("Failed to open ccake.toml file for reading!");

        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .expect("Failed to read contents of ccake.toml file!");

        toml::from_str(&file_content).expect("Failed to deserialize content from ccake.toml file!")
    }
}

impl Config {
    pub fn write(&self, sub_path: &Option<&String>) {
        let config_as_str = toml::to_string(&self).expect("Failed to serialize config to string!");

        if let Some(path) = sub_path {
            let ccake_path = format!("{}{}{}", path, "/", CCAKE_CONFIG_FILE_NAME);
            std::fs::create_dir_all(path)
                .expect("Failed to create directories to ccake.toml path!");
            std::fs::write(ccake_path, config_as_str).expect("Failed to write to ccake.toml file!");
            return;
        }

        std::fs::write(CCAKE_CONFIG_FILE_NAME, config_as_str)
            .expect("Failed to write to ccake.toml file!");
    }

    pub fn out_dir(&self) -> String {
        self.project_properties
            .out_dir
            .clone()
            .unwrap_or_else(|| "out".to_string())
    }

    pub fn src_dir(&self) -> String {
        self.project_properties
            .src_dir
            .to_owned()
            .unwrap_or_else(|| "src".to_string())
    }

    pub fn compiler_dir(&self) -> Option<&String> {
        self.compiler_properties
            .as_ref()
            .and_then(|f| f.compiler_dir.as_ref())
    }

    pub fn compiler_args(&self) -> Option<&String> {
        self.compiler_properties
            .as_ref()
            .and_then(|f| f.compiler_args.as_ref())
    }
}
