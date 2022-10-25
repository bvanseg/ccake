use crate::lib;

pub fn new() -> clap::Command {
    clap::Command::new("install")
        .about("Installs a tool or library.")
        .arg(
            clap::Arg::new("tool-library-name")
                .required(true)
                .help("The name of the tool or library to install."),
        )
}

pub fn exec(arg_matches: &clap::ArgMatches) {
    lib::project::install(arg_matches.get_one::<String>("tool-library-name"));
}
