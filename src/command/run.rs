// use crate::lib;
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
                .trailing_var_arg(true),
        )
}

pub fn exec(arg_matches: &ArgMatches) {
    match arg_matches.get_one::<String>("command") {
        Some(cmd) => {
            // Ensure that command exists in ccake.toml
            println!("TODO");
            // Handle extra args passed to script
            let extra_args: Vec<_> = arg_matches
                .get_many::<&str>("args")
                .map(|mut args| {
                    let arg_separator = args.next();
                    // Ignore extra args if not explicitly passed through "--" separator
                    if arg_separator == Some(&"--") {
                        args.into_iter().map(|&s| s).collect()
                    } else {
                        vec![]
                    }
                })
                .unwrap_or(vec![]);
            let rest: String = extra_args.join(" ");
        }
        None => unreachable!("Command must be called with the name of a command"),
    }
}
