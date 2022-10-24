use clap::{crate_authors, crate_description, crate_name, crate_version, Command};

#[cfg(windows)]
use fansi::windows::enable_ansi_support;

mod command;
mod lib;

fn main() {
    let command = Command::new(crate_name!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .version(crate_version!())
        .subcommand(command::new::new())
        .subcommand(command::init::new())
        .subcommand(command::build::new())
        .subcommand(command::install::new())
        .subcommand(command::configure::new());

    let matches = command.get_matches();

    #[cfg(windows)]
    enable_ansi_support().expect("Failed to enable ansi support for Windows!"); // TODO: Remove this line.

    match matches.subcommand() {
        Some(("new", arg_matches)) => command::new::exec(arg_matches),
        Some(("init", _)) => command::init::exec(),
        Some(("build", arg_matches)) => command::build::exec(arg_matches),
        Some(("install", arg_matches)) => command::install::exec(arg_matches),
        Some(("configure", arg_matches)) => command::configure::exec(arg_matches),
        Some((_, _)) => (),
        None => (),
    }
}
