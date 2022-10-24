use crate::lib;

pub fn new() -> clap::Command {
    clap::Command::new("configure")
        .about("Configure CCake's global properties.")
        .arg(
            clap::Arg::new("default-c-compiler-dir")
                .help("The path to the default C compiler to be used for building projects.")
                .long("c-compiler-dir"),
        )
        .arg(
            clap::Arg::new("default-cpp-compiler-dir")
                .help("The path to the default C++ compiler to be used for building projects.")
                .long("cpp-compiler-dir"),
        )
}

pub fn exec(arg_matches: &clap::ArgMatches) {
    lib::configure::configure(arg_matches);
}
