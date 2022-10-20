use clap::crate_version;

use crate::config::write_config;
use crate::terminal::prompt::prompt;
use crate::{config, HELLO_C, HELLO_CPP};

pub fn initialize_project(sub_path: Option<&String>) {
    let project_name = prompt("Project Name:");
    let project_version = prompt("Version:");
    let project_authors = prompt("Authors:");

    let config = config::Config {
        project_properties: config::ProjectProperties {
            project_name: project_name.trim().to_string(),
            project_version: project_version.trim().to_string(),
            authors: Some(project_authors
                .trim()
                .split(',')
                .map(|f| f.trim().to_string())
                .collect()),
            ccake_version: crate_version!().to_string(),

            language: "C++".to_string(),
            project_type: config::ProjectType::Binary,
            src_dir: None,
            out_dir: None,
        },
        compiler_properties: config::CompilerProperties {
            compiler_dir: None,
            compiler_args: None,
        },
    };

    write_project_files(&config, sub_path);
}

fn write_project_files(config: &config::Config, sub_path: Option<&String>) {
    write_config(config, &sub_path);
    write_hello_world(config, &sub_path);
}

fn write_hello_world(config: &config::Config, sub_path: &Option<&String>) {
    let src_dir = config
        .project_properties
        .src_dir
        .to_owned()
        .unwrap_or_else(|| "src".to_string());

    let lowercase_language = config.project_properties.language.to_lowercase();
    let lang_str = lowercase_language.as_str();

    let main_file_name = match lang_str {
        "c" => "main.c",
        "c++" | "cpp" => "main.cpp",
        _ => panic!("Unknown project language specified in ccake.toml!"),
    };

    let main_file_content = match lang_str {
        "c" => HELLO_C,
        "c++" | "cpp" => HELLO_CPP,
        _ => panic!("Unknown project language specified in ccake.toml!"),
    };

    if let Some(path) = sub_path {
        let main_file_parent_path = format!("{}{}{}", path, "/", src_dir);
        std::fs::create_dir_all(&main_file_parent_path)
            .expect("Failed to create directories to main.cxx path!");
        let main_file_path = format!("{}{}{}", main_file_parent_path, "/", main_file_name);
        std::fs::write(main_file_path, main_file_content)
            .expect("Failed to write to main.cxx file!");
        return;
    }

    std::fs::create_dir_all(&src_dir).expect("Failed to create directories to main.cxx path!");
    let main_file_path = format!("{}{}{}", &src_dir, "/", main_file_name);
    std::fs::write(main_file_path, main_file_content).expect("Failed to write to main.cxx file!");
}
