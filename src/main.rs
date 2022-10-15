use std::env;
use std::io;
use std::io::Write;

use crate::terminal::ansi;
use crate::windows::enable_ansi_support;

mod terminal;
mod windows;

fn main() {
    // By default, ansi is not supported in Windows terminals. It must be enabled through Window's native API.
    enable_ansi_support().expect("Failed to enable ansi support for Windows!");

    // Collect command line arguments ran through ccake.
    let args: Vec<String> = env::args().collect();

    // Output args in debug.
    // dbg!(args);

    // TODO: Check command arguments.

    let ansi_project_name = ansi::ANSI_CHOICE_STYLE.apply("Project Name:");

    // TODO: Move the lines below this one out to some helper function for receiving inline, labeled input from a user.

    // Prompt for a project name and flush to stdout.
    print!("{} ", ansi_project_name);
    io::stdout()
        .flush()
        .expect("Failed to flush standard output buffer!");

    // Capture input in response to previous project name prompt.s
    let buffer = &mut String::new();
    io::stdin()
        .read_line(buffer)
        .expect("Unable to read input.");
}
