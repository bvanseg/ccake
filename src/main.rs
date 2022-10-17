use crate::command::build_project::build_project;
use crate::command::configure::configure;
use crate::command::new_project::initialize_project;
use crate::command::install::install;
use crate::terminal::windows::enable_ansi_support;

use clap::{Command, Arg, crate_authors, crate_description, crate_name, crate_version};

mod command;
mod config;
mod settings;
mod terminal;

static HELLO_C: &str = include_str!("../res/hello.c");
static HELLO_CPP: &str = include_str!("../res/hello.cpp");

fn main() {
    let command = Command::new(crate_name!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .version(crate_version!())
        .subcommand(
            Command::new("new")
                .about("Creates a new C/C++ project.")
                .arg(
                    Arg::new("folder")
                        .required(true)
                        .help("The folder to create for the new project.")
                )
        )
        .subcommand(
            Command::new("init")
                .about("Creates a new C/C++ project within the current directory.")
        )
        .subcommand(
            Command::new("build")
                .about("Builds the existing C/C++ project within the current directory.")
        )
        .subcommand(
            Command::new("install")
                .about("Installs a tool or library.")
                .arg(
                    Arg::new("tool-library-name")
                        .required(true)
                        .help("The name of the tool or library to install.")
                )
        )
        .subcommand(
            Command::new("configure")
                .about("Configure CCake's global properties.")
                .arg(
                    Arg::new("default-compiler-dir")
                        .help("The path to the default compiler to be used for building projects.")
                        .short('c')
                        .long("compiler-dir")
                )
        );

    let matches = command.get_matches();

    enable_ansi_support().expect("Failed to enable ansi support for Windows!"); // TODO: Remove this line.

    match matches.subcommand() {
        Some(("new", arg_matches)) => initialize_project(arg_matches.get_one::<String>("folder")),
        Some(("init", _)) => initialize_project(None),
        Some(("build", _)) => build_project(),
        Some(("install", arg_matches)) => install(arg_matches.get_one::<String>("tool-library-name")),
        Some(("configure", arg_matches)) => configure(arg_matches),
        Some((_, _)) => (),
        None => ()
    }
}
