use std::fmt::Display;
use std::io;
use std::io::Write;

/**
 * Prompts a user for input, and returns the user's input back as a [String] type.
 */
pub fn prompt<T: Display>(displayable: T) -> String {
    // Prompt for a project name and flush to stdout.
    print!("{} ", displayable);
    io::stdout()
        .flush()
        .expect("Failed to flush standard output buffer!");

    // Capture input in response to previous project name prompt.
    let buffer = &mut String::new();
    io::stdin()
        .read_line(buffer)
        .expect("Unable to read input.");

    return buffer.to_string();
}
