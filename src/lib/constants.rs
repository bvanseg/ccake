pub const CONFIG_FILE_NAME: &str = "ccake.toml";

#[cfg(unix)]
pub const INSTALL_SCRIPT: &str = include_str!("../../res/install.sh");

#[cfg(windows)]
pub const INSTALL_SCRIPT: &str = include_str!("../../res/install.ps1");

pub const HELLO_C: &str = include_str!("../../res/hello.c");
pub const HELLO_CPP: &str = include_str!("../../res/hello.cpp");
