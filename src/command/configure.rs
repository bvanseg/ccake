use clap::ArgMatches;

use crate::settings::{read_settings, write_settings};

pub fn configure(arg_matches: &ArgMatches) {
    let mut current_settings = &mut read_settings();

    if let Ok(Some(default_c_compiler_dir)) = arg_matches.try_get_one::<String>("default-c-compiler-dir") {
        current_settings.default_c_compiler_dir = default_c_compiler_dir.to_string();
    }

    if let Ok(Some(default_cpp_compiler_dir)) = arg_matches.try_get_one::<String>("default-cpp-compiler-dir") {
        current_settings.default_cpp_compiler_dir = default_cpp_compiler_dir.to_string();
    }

    write_settings(current_settings);
}
