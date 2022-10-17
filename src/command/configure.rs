use clap::ArgMatches;

use crate::settings::{read_settings, write_settings};

pub fn configure(arg_matches: &ArgMatches) {
    let mut current_settings = &mut read_settings();

    if let Ok(default_compiler_dir) = arg_matches.try_get_one::<String>("default-compiler-dir") {
        current_settings.default_compiler_dir = default_compiler_dir.unwrap().to_string();
    }

    write_settings(current_settings);
}