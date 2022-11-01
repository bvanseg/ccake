use crate::lib::project::Config;
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
    match arg_matches.get_one::<String>("command") {
        Some(cmd) => {
            let extra_args: Vec<&String> = arg_matches
                .get_many::<String>("args")
                .map(|args| args.into_iter().collect())
                .unwrap_or(vec![]);
            println!("args: {:?}", extra_args);
            Config::run(cmd, extra_args);
        }
        None => unreachable!("Command must be called with the name of a command"),
    }
}
