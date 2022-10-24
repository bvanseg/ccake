use crate::lib;

pub fn new() -> clap::Command {
    clap::Command::new("init").about("Creates a new C/C++ project within the current directory.")
}

pub fn exec() {
    lib::project::init(None);
}
