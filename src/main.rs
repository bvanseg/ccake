use std::env;

use crate::new_project::create_new_project;
use crate::terminal::ansi;
use crate::terminal::prompt::{prompt};
use crate::windows::enable_ansi_support;

mod new_project;
mod terminal;
mod windows;

fn main() {
    // By default, ansi is not supported in Windows terminals. It must be enabled through Window's native API.
    enable_ansi_support().expect("Failed to enable ansi support for Windows!");

    // Collect command line arguments ran through ccake.
    let args: Vec<String> = env::args().collect();

    // Output args in debug.
    // dbg!(args);

    for argument in args {
        let argument_as_str = argument.as_str();
        match argument_as_str {
            "new" => create_new_project(),
            "init" => (), // TODO
            "build" => (), // TODO
            _ => ()
        }
    }
}
