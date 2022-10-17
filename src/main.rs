use crate::command::build_project::build_project;
use crate::command::new_project::initialize_project;
use crate::terminal::windows::enable_ansi_support;

use clap::{Command, Arg, crate_authors, crate_description, crate_name, crate_version};

mod command;
mod config;
mod terminal;

const CCAKE_VERSION: &str = "1.0.0";

static HELLO_C: &str = include_str!("../res/hello.c");
static HELLO_CPP: &str = include_str!("../res/hello.cpp");

fn main() {
    let command = Command::new(crate_name!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .version(crate_version!())
        .subcommand(
            Command::new("new")
                .arg(
                    Arg::new("folder")
                        .required(true)
                        .help("The folder to create for the new project.")
                )
        )
        .subcommand(
            Command::new("init")
        )
        .subcommand(
            Command::new("build")
        );

    let matches = command.get_matches();

    enable_ansi_support().expect("Failed to enable ansi support for Windows!");

    match matches.subcommand() {
        Some(("new", arg_matches)) => initialize_project(arg_matches.get_one::<String>("folder")),
        Some(("init", _)) => initialize_project(None),
        Some(("build", _)) => build_project(),
        Some((_, _)) => (),
        None => ()
    }
}
