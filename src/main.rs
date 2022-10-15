use std::env;

use crate::command::build_project::build_project;
use crate::command::new_project::initialize_project;
use crate::terminal::windows::enable_ansi_support;

mod command;
mod config;
mod terminal;

const CCAKE_VERSION: &str = "1.0.0";

fn main() {
    // By default, ansi is not supported in Windows terminals. It must be enabled through Window's native API.
    enable_ansi_support().expect("Failed to enable ansi support for Windows!");

    // Collect command line arguments ran through ccake.
    let mut args = env::args();

    while let Some(argument_as_str) = args.next() {
        match argument_as_str.as_str() {
            "new" => initialize_project(args.next()),
            "init" => initialize_project(None),
            "build" => build_project(),
            _ => ()
        }
    }
}
