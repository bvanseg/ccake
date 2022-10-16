use std::io::Write;
use walkdir::WalkDir;
use crate::{terminal::ansi, config};

pub fn build_project() {
    let config = config::read_config();

    run_compiler(&config);
}

fn collect_c_file_paths(config: &config::Config, src_dir: &String) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    let target_extension = match config.project_properties.language.to_lowercase().as_str() {
        "c" => ".c",
        "c++" | "cpp" => ".cpp",
        _ => panic!("Unknown project language specified in ccake.toml!")
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

    let out_file = match config.project_properties.project_type {
        config::ProjectType::Binary => "app.exe",
        config::ProjectType::Library => "library.dll"
    };

    let out_dir = &config.project_properties.out_dir.to_owned().unwrap_or_else(|| "out".to_string());

    // TODO: There has got to be a better/safer way of doing this...
    let out_file_path = std::path::Path::new("")
        .join(out_dir);

        std::fs::create_dir_all(&out_file_path).expect("Failed to create output directories for compiler output file!");

    let out_file_path = out_file_path.join(out_file);

    let out_file_path_str = out_file_path.into_os_string().into_string().unwrap();


    let mut project_args: Vec<String> = match config.project_properties.project_type {
        config::ProjectType::Binary => vec!["-o", out_file_path_str.as_str()], // TODO: This shouldn't be .exe.
        // TODO: For library building, there should be a way to distinguish static and dynamic libraries.
        config::ProjectType::Library => vec!["-shared", "-o", out_file_path_str.as_str()] // TODO: This shouldn't be .dll.
    }.into_iter().map(String::from).collect();
    
    // Collect C source files for inputting into the compiler.
    let src_dir = &config.project_properties.src_dir.to_owned().unwrap_or_else(|| "src".to_string());
    let c_files = &mut collect_c_file_paths(&config, src_dir);
    project_args.append(c_files);

    // Try to append compiler arguments from config's compiler_args property.
    if let Some(compiler_args) = &config.compiler_properties.compiler_args {
        let mut split_args: Vec<String> = compiler_args.split_whitespace().into_iter().map(String::from).collect();
        project_args.append(&mut split_args);
    }

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