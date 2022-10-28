use clap::crate_version;

use crate::lib::constants;
use crate::lib::project::config::Config;

pub fn initialize_project(sub_path: Option<&String>) {
    let path = match sub_path {
        Some(path) => std::path::Path::new(path).join("ccake.toml"),
        None => std::path::Path::new("ccake.toml").to_path_buf(),
    };

    if let Ok(true) = path.try_exists() {
        error!("Project already exists in the target directory.");
        std::process::exit(-1);
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
    let project_name = dialoguer::Input::<String>::new()
        .with_prompt("Project Name")
        .default("Example Project".to_string())
        .interact_text()
        .unwrap_or_else(|_| "Example Project".to_string());

    let project_language = dialoguer::Input::<String>::new()
        .with_prompt("Language (C/C++)")
        .validate_with(|input: &String| -> Result<(), String> {
            let processed_input = input.trim().to_lowercase();

            if processed_input.is_empty() {
                return Ok(());
            }

            return match processed_input.as_str() {
                "c" | "c++" => Ok(()),
                "cpp" => Ok(()),
                _ => Err("Value must be either 'C' or 'C++'!".to_string()),
            };
        })
        .default("C++".to_string())
        .interact_text()
        .unwrap_or_else(|_| "C++".to_string());

    let project_version = dialoguer::Input::<String>::new()
        .with_prompt("Version")
        .default("0.1.0".to_string())
        .interact_text()
        .unwrap_or_else(|_| "0.1.0".to_string());

    let project_authors = dialoguer::Input::<String>::new()
        .with_prompt("Authors")
        .default("John Doe".to_string())
        .interact_text()
        .unwrap_or_else(|_| "John Doe".to_string());

    (
        project_name,
        project_language.as_str().to_uppercase(),
        project_version,
        project_authors,
    )
}

fn write_project_files(config: &Config, sub_path: Option<&String>) {
    config.write(&sub_path);
    write_hello_world(config, &sub_path);
}

fn write_hello_world(config: &Config, sub_path: &Option<&String>) {
    let src_dir = config
        .src_dirs()
        .into_iter()
        .next()
        .unwrap_or_else(|| "src".to_string());

    let lowercase_language = config.project_properties.language.to_lowercase();
    let lang_str = lowercase_language.as_str();

    let (main_file_name, main_file_content) = match lang_str {
        "c" => ("main.c", constants::HELLO_C),
        "c++" | "cpp" => ("main.cpp", constants::HELLO_CPP),
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
