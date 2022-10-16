use std::io::Write;
use walkdir::WalkDir;
use crate::{terminal::ansi, config};

pub fn build_project() {
    let config = config::read_config();

    run_compiler(&config);
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

fn run_compiler(config: &config::Config) {
    // TODO: Prepend output dir to file names here.
    // TODO: These file names should be based on the ccake.toml properties.
    let mut project_args: Vec<String> = match config.project_properties.project_type {
        config::ProjectType::Binary => vec!["-o", "app.exe"], // TODO: This shouldn't be .exe.
        // TODO: For library building, there should be a way to distinguish static and dynamic libraries.
        config::ProjectType::Library => vec!["-shared", "-o", "library.dll"] // TODO: This shouldn't be .dll.
    }.into_iter().map(String::from).collect();
    
    // Collect C source files for inputting into the compiler.
    let c_files = &mut collect_c_file_paths(&config.project_properties.src_dir);
    project_args.append(c_files);

    // Run the compiler executable.
    let output = std::process::Command::new(&config.compiler_properties.compiler_dir)
        .args(project_args)
        .output()
        .expect("failed to execute compiler process!");

    // If there was standard output from the compiler, emit it.
    let standard_output = &String::from_utf8(output.stdout).expect("Failed to convert output stdout to String!");
    let ansi_standard_output = ansi::ANSI_DEFAULT_STYLE.apply(standard_output);
    std::io::stdout().write_all(&ansi_standard_output.as_string().as_bytes()).unwrap();

    // If there was erroroneous output from the compiler, emit it.
    let error_output = &String::from_utf8(output.stderr).expect("Failed to convert output stderr to String!");
    let ansi_err_output = ansi::ANSI_ERROR_STYLE.apply(error_output);
    std::io::stderr().write_all(&ansi_err_output.as_string().as_bytes()).unwrap();
}