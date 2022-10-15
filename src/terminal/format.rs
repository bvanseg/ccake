
pub fn print_command_help_line(command_arg: &str, command_hint: &str) {
    println!("{:preident$}{}\t{}", "", command_arg, command_hint, preident=1);
}