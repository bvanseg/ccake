use clap::ArgMatches;

use crate::lib::global::settings::Settings;

pub fn configure(arg_matches: &ArgMatches) {
    let mut current_settings = &mut Settings::read();

    if let Ok(Some(default_c_compiler_dir)) =
        arg_matches.try_get_one::<String>("default-c-compiler-dir")
    {
        current_settings.default_c_compiler_dir = default_c_compiler_dir.to_string();
    }

    if let Ok(Some(default_cpp_compiler_dir)) =
        arg_matches.try_get_one::<String>("default-cpp-compiler-dir")
    {
        current_settings.default_cpp_compiler_dir = default_cpp_compiler_dir.to_string();
    }

    Settings::write(current_settings);
}
