use crate::config;
use crate::config::write_config;

use super::prompt;
use super::ansi;

pub fn initialize_project(sub_path: Option<String>) {
    let ansi_project_name = ansi::ANSI_CHOICE_STYLE.apply("Project Name:");
    let ansi_project_version = ansi::ANSI_CHOICE_STYLE.apply("Version:");
    let ansi_project_authors = ansi::ANSI_CHOICE_STYLE.apply("Authors:");

    let project_name = prompt(ansi_project_name);
    let project_version = prompt(ansi_project_version);
    let project_authors = prompt(ansi_project_authors);

    let config = config::Config {
        project_properties: config::ProjectProperties {
            project_name: project_name.trim().to_string(),
            project_version: project_version.trim().to_string(),
            authors: project_authors.trim().split(',').map(|f| f.trim().to_string()).collect(),
            ccake_version: super::CCAKE_VERSION.to_string(),

            src_dir: "src".to_string()
        },
        compiler_properties: config::CompilerProperties {
            compiler_dir: "/path/to/compiler".to_string()
        }
    };

    write_config(&config, sub_path);
}