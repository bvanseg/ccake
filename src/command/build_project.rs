use crate::{config, settings, terminal::ansi::{ANSI_ERROR_STYLE}};
use fansi::{string::AnsiString, style::AnsiStyle};
use std::{io::Write, path::Path};
use walkdir::WalkDir;

pub fn build_project() {
    let config = config::read_config();

    // TODO: Determine file extension based on platform + project type (binary/library(static/dynamic)).
    // TODO: These file names should be based on the ccake.toml properties.
    let out_file = match config.project_properties.project_type {
        config::ProjectType::Binary => "app.exe",
        config::ProjectType::Library => "library.dll",
    };

    let out_dir = &config
        .project_properties
        .out_dir
        .to_owned()
        .unwrap_or_else(|| "out".to_string());

    // TODO: There has got to be a better/safer way of doing this...
    let out_file_path = std::path::Path::new("").join(out_dir);

    std::fs::create_dir_all(&out_file_path)
        .expect("Failed to create output directories for compiler output file(s)!");

    let out_file_path = out_file_path.join(out_file);

    let out_file_path_str = out_file_path.into_os_string().into_string().unwrap();

    // Get the working compiler directory.
    let working_compiler_dir = compute_working_compiler_dir(&config);

    let project_args = compute_compiler_args(&config, out_file_path_str);

    // Run the compiler executable.
    execute_compiler(working_compiler_dir, project_args);
}

fn collect_cxx_file_paths(config: &config::Config, src_dir: &String) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    let target_extension = match config.project_properties.language.to_lowercase().as_str() {
        "c" => ".c",
        "c++" | "cpp" => ".cpp",
        _ => panic!("Unknown project language specified in ccake.toml!"),
    };

    let walker = WalkDir::new(src_dir)
        .into_iter()
        .filter_entry(|p| {
            let path_str = p.file_name().to_string_lossy();
            path_str.starts_with(src_dir) || path_str.ends_with(target_extension) == true
        })
        .filter_map(|r| r.ok());

    for entry in walker {
        if entry.path().is_dir() {
            continue;
        }

        if let Some(path_str) = entry.path().to_str() {
            vec.push(path_str.to_string());
        }
    }

    return vec;
}

fn compute_compiler_args(config: &config::Config, out_file_path_str: String) -> Vec<String> {

    let mut project_args: Vec<String> = match config.project_properties.project_type {
        config::ProjectType::Binary => vec!["-o", out_file_path_str.as_str()],
        // TODO: For library building, there should be a way to distinguish static and dynamic libraries.
        config::ProjectType::Library => vec!["-shared", "-o", out_file_path_str.as_str()],
    }
    .into_iter()
    .map(String::from)
    .collect();

    // Collect C source files for inputting into the compiler.
    let src_dir = &config
        .project_properties
        .src_dir
        .to_owned()
        .unwrap_or_else(|| "src".to_string());
    let c_files = &mut collect_cxx_file_paths(&config, src_dir);
    project_args.append(c_files);

    // Try to append compiler arguments from config's compiler_args property.
    if let Some(compiler_args) = &config.compiler_properties.compiler_args {
        let mut split_args: Vec<String> = compiler_args
            .split_whitespace()
            .into_iter()
            .map(String::from)
            .collect();
        project_args.append(&mut split_args);
    }

    return project_args;
}

fn compute_working_compiler_dir(config: &config::Config) -> Option<String> {
    // First try to use the project compiler directory.
    let mut working_compiler_dir = config.compiler_properties.compiler_dir.clone();

    match &working_compiler_dir {
        // If the project compiler path does not exist, try checking the default compiler path.
        Some(project_compiler_dir) => if !Path::new(project_compiler_dir).exists() {
            read_default_compiler_dir(config, &mut working_compiler_dir);
        }
        None => read_default_compiler_dir(config, &mut working_compiler_dir)
    }

    return working_compiler_dir;
}

fn read_default_compiler_dir(config: &config::Config, working_compiler_dir: &mut Option<String>) {
    let settings = settings::read_settings();
    let default_ccake_compiler_dir = match config.project_properties.language.to_lowercase().as_str() {
        "c" => settings.default_c_compiler_dir,
        "c++" | "cpp" => settings.default_cpp_compiler_dir,
        _ => panic!("Unknown project language specified in ccake.toml!"),
    };
    if !Path::new(&default_ccake_compiler_dir).exists() {
        panic!("Failed to get a working compiler path for building!");
    } else {
        *working_compiler_dir = Some(default_ccake_compiler_dir);
    }
}

fn execute_compiler(working_compiler_dir: Option<String>, project_args: Vec<String>) {
    let output = std::process::Command::new(&working_compiler_dir.unwrap())
        .args(project_args)
        .output()
        .expect("failed to execute compiler process!");

    handle_compiler_stdout(&output.stdout, &[]);
    handle_compiler_stdout(&output.stderr, &ANSI_ERROR_STYLE);
}

fn handle_compiler_stdout(output: &Vec<u8>, style: &[AnsiStyle]) {
    // If there was standard output from the compiler, emit it.
    let output_str =
        &String::from_utf8(output.to_vec()).expect("Failed to convert output stdout to String!");
    let ansi_err_output = AnsiString::with_styles_arr(output_str, style);
    std::io::stderr()
        .write_all(&ansi_err_output.to_string().as_bytes())
        .unwrap();
}
