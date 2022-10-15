use std::io::Write;
use walkdir::WalkDir;
use crate::{terminal::ansi, config};

pub fn build_project() {
    let config = config::read_config();

    let c_files = collect_c_file_paths(&config.project_properties.src_dir);

    run_compiler(&config.compiler_properties.compiler_dir, &c_files);
}

fn collect_c_file_paths(src_dir: &String) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    let walker = WalkDir::new(src_dir)
        .into_iter()
        .filter_entry(|p| {
            let path_str = p.file_name().to_string_lossy();
            path_str.starts_with(src_dir) || path_str.ends_with(".c") == true
        })
        .filter_map(|r| r.ok());

    for entry in walker {
        if entry.path().is_dir() {
            continue
        }

        if let Some(path_str) = entry.path().to_str() {
            vec.push(path_str.to_string());
        }
    }

    return vec;
}

fn run_compiler(compiler_dir: &String, c_files: &Vec<String>) {
    let output = std::process::Command::new(compiler_dir)
        .args(["".to_string()].iter().chain(c_files))
        .output()
        .expect("failed to execute compiler process!");

    let standard_output = &String::from_utf8(output.stdout).expect("Failed to convert output stdout to String!");
    let ansi_standard_output = ansi::ANSI_ERROR_STYLE.apply(standard_output);
    std::io::stdout().write_all(&ansi_standard_output.as_string().as_bytes()).unwrap();

    let error_output = &String::from_utf8(output.stderr).expect("Failed to convert output stderr to String!");
    let ansi_err_output = ansi::ANSI_ERROR_STYLE.apply(error_output);
    std::io::stderr().write_all(&ansi_err_output.as_string().as_bytes()).unwrap();
}