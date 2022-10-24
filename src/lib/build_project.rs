use crate::{
    config::{Config, ProjectType},
    settings::Settings,
    terminal::ansi::ANSI_ERROR_STYLE,
};
use clap::ArgMatches;
use fansi::{string::AnsiString, style::AnsiStyle};
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

pub fn build_project(arg_matches: &ArgMatches) {
    let config = Config::read();

    let out_dir = &config.out_dir();

    if arg_matches.get_flag("clean") {
        if let Err(e) = std::fs::remove_dir_all(&out_dir) {
            match e.kind() {
                std::io::ErrorKind::NotFound => (), // If the dir does not exist, no need to handle.
                _ => panic!("Failed to delete '{}' directory! Error: {:?}", out_dir, e),
            }
        }
    }

    let out_file = match config.project_properties.project_type {
        ProjectType::Binary => {
            if cfg!(windows) {
                "app.exe"
            } else {
                "app.AppImage"
            }
        }
        ProjectType::Library => {
            if arg_matches.get_flag("static-library") {
                if cfg!(windows) {
                    "library.lib"
                } else {
                    "library.a"
                }
            } else if cfg!(windows) {
                "library.dll"
            } else {
                "library.so"
            }
        }
    };

    let out_file_path = std::path::Path::new(&out_dir);

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

fn collect_cxx_file_paths(config: &Config, src_dir: &String) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();

    let target_extension = match config.project_properties.language.to_lowercase().as_str() {
        "c" => ".c",
        "c++" | "cpp" => ".cpp",
        _ => panic!("Unknown project language specified in ccake.toml!"),
    };

    let walker = WalkDir::new(src_dir)
        .into_iter()
        .filter_entry(|p| {
            let path_str = p.file_name().to_string_lossy();
            path_str.starts_with(src_dir) || path_str.ends_with(target_extension)
        })
        .filter_map(|r| r.ok());

    for entry in walker {
        if entry.path().is_dir() {
            continue;
        }

        if let Some(path_str) = entry.path().to_str() {
            paths.push(path_str.to_string());
        }
    }

    paths
}

fn compute_compiler_args(config: &Config, out_file_path: String) -> Vec<String> {
    let mut project_args: Vec<String> = match config.project_properties.project_type {
        ProjectType::Binary => vec!["-o", out_file_path.as_str()],
        // TODO: For library building, there should be a way to distinguish static and dynamic libraries.
        ProjectType::Library => vec!["-shared", "-o", out_file_path.as_str()],
    }
    .into_iter()
    .map(String::from)
    .collect();

    // Collect C source files for inputting into the compiler.
    let src_dir = config.src_dir();
    let c_files = &mut collect_cxx_file_paths(config, &src_dir);
    project_args.append(c_files);

    // Try to append compiler arguments from config's compiler_args property.
    if let Some(compiler_args) = config.compiler_args() {
        let mut split_args: Vec<String> = compiler_args
            .split_whitespace()
            .into_iter()
            .map(String::from)
            .collect();
        project_args.append(&mut split_args);
    }

    project_args
}

/// Tries to get a valid compiler directory, where 'valid' is a configured
/// and existing directory
fn compute_working_compiler_dir(config: &Config) -> String {
    // First try to use the project compiler directory.
    if let Some(project_compiler_dir) = config.compiler_dir() {
        if Path::new(&project_compiler_dir).exists() {
            return project_compiler_dir.to_owned();
        }
    }

    // If project compiler directory is invalid, try using ccake's default settings
    let settings = Settings::read();
    let default_ccake_compiler_dir = read_default_compiler_dir(config, &settings);
    if Path::new(&default_ccake_compiler_dir).exists() {
        return default_ccake_compiler_dir;
    }

    panic!("Failed to get a working compiler path for building!")
}

fn read_default_compiler_dir(config: &Config, settings: &Settings) -> String {
    match config.project_properties.language.to_lowercase().as_str() {
        "c" => &settings.default_c_compiler_dir,
        "c++" | "cpp" => &settings.default_cpp_compiler_dir,
        _ => panic!("Unknown project language specified in ccake.toml!"),
    }
    .to_owned()
}

fn execute_compiler(working_compiler_dir: String, project_args: Vec<String>) {
    let output = std::process::Command::new(&working_compiler_dir)
        .args(project_args)
        .output()
        .expect("failed to execute compiler process!");

    handle_compiler_stdout(&output.stdout, &[]);
    handle_compiler_stdout(&output.stderr, &ANSI_ERROR_STYLE);
}

fn handle_compiler_stdout(output: &[u8], style: &[AnsiStyle]) {
    // If there was standard output from the compiler, emit it.
    let output_str =
        &String::from_utf8(output.to_vec()).expect("Failed to convert output stdout to String!");
    let ansi_err_output = AnsiString::with_styles_arr(output_str, style);
    std::io::stderr()
        .write_all(ansi_err_output.to_string().as_bytes())
        .unwrap();
}
