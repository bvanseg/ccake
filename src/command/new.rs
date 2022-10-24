use crate::lib;

pub fn new() -> clap::Command {
    clap::Command::new("new")
        .about("Creates a new C/C++ project.")
        .arg(
            clap::Arg::new("folder")
                .required(true)
                .help("The folder to create for the new project."),
        )
}

pub fn exec(arg_matches: &clap::ArgMatches) {
    lib::project::init(arg_matches.get_one::<String>("folder"));
}
