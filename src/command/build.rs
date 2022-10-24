use crate::lib;

pub fn new() -> clap::Command {
    clap::Command::new("build")
        .about("Builds the existing C/C++ project within the current directory.")
        .arg(
            clap::Arg::new("static-library")
                .help("Specifies that a static library should be built from the current project.")
                .short('s')
                .long("static")
                .action(clap::ArgAction::SetTrue),
        )
}

pub fn exec(arg_matches: &clap::ArgMatches) {
    lib::build_project::build_project(arg_matches);
}
