use clap::crate_version;

use crate::terminal;
use crate::terminal::ansi::error;
use crate::terminal::prompt::prompt;
use crate::{config::Config, HELLO_C, HELLO_CPP};

pub fn initialize_project(sub_path: Option<&String>) {
    let path = match sub_path {
        Some(path) => std::path::Path::new(path).join("ccake.toml"),
        None => std::path::Path::new("ccake.toml").to_path_buf(),
    };

    if let Ok(true) = path.try_exists() {
        terminal::ansi::error("Project already exists in the target directory.");
        return;
    }

    let (project_name, project_language, project_version, project_authors) =
        prompt_user_for_project_details();

    let config = Config::new(
        project_name,
        project_version,
        project_authors,
        crate_version!().to_string(),
        project_language,
    );
    write_project_files(&config, sub_path);
}

fn prompt_user_for_project_details() -> (String, String, String, String) {
    let project_name = prompt("Project Name:").unwrap_or_else(|| "Example".to_string());
    let project_language = loop {
        let input =
            prompt("Language (C/C++, default is C++):").unwrap_or_else(|| "C++".to_string());
        let processed_input = input.trim().to_lowercase();

        if processed_input.is_empty() {
            break "C++".to_string();
        }

        match processed_input.as_str() {
            "c" | "c++" => break processed_input.to_uppercase(),
            "cpp" => break "C++".to_string(),
            _ => {
                error("Value must be either 'C' or 'C++', please try again: ");
                continue;
            }
        }
    };
    let project_version = prompt("Version:").unwrap_or_else(|| "1.0.0".to_string());
    let project_authors =
        prompt("Authors:").unwrap_or_else(|| "[Author Name] <[email]>".to_string());

    (
        project_name,
        project_language,
        project_version,
        project_authors,
    )
}

fn write_project_files(config: &Config, sub_path: Option<&String>) {
    config.write(&sub_path);
    write_hello_world(config, &sub_path);
}

fn write_hello_world(config: &Config, sub_path: &Option<&String>) {
    let src_dir = config.src_dir();

    let lowercase_language = config.project_properties.language.to_lowercase();
    let lang_str = lowercase_language.as_str();

    let (main_file_name, main_file_content) = match lang_str {
        "c" => ("main.c", HELLO_C),
        "c++" | "cpp" => ("main.cpp", HELLO_CPP),
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
