use crate::lib;
use clap::{arg, Arg, ArgMatches, Command};

pub fn new() -> Command {
    Command::new("run")
        .about("Runs a command defined in the ccake.toml file")
        .arg(
            Arg::new("command")
                .required(true)
                .help("Name of command in ccake.toml"),
        )
        .arg(
            arg!(<args> ... "extra args to pass to script")
                .required(false)
                .last(true),
        )
}

pub fn exec(arg_matches: &ArgMatches) {
    let cmd = arg_matches.get_one::<String>("command").unwrap();
    let extra_args: Vec<&String> = arg_matches
        .get_many::<String>("args")
        .map(|args| args.into_iter().collect())
        .unwrap_or(vec![]);
    lib::project::run_command(cmd, extra_args);
}
