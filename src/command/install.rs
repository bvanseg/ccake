<<<<<<< HEAD
use crate::lib;

pub fn new() -> clap::Command {
    clap::Command::new("install")
        .about("Installs a tool or library.")
        .arg(
            clap::Arg::new("tool-library-name")
                .required(true)
                .help("The name of the tool or library to install."),
        )
}

pub fn exec(arg_matches: &clap::ArgMatches) {
    lib::project::install(arg_matches.get_one::<String>("tool-library-name"));
=======
use std::io::Write;

#[cfg(unix)]
static INSTALL_SCRIPT: &str = include_str!("../../res/install.sh");

#[cfg(windows)]
static INSTALL_SCRIPT: &str = include_str!("../../res/install.ps1");

pub fn install(tool_library_name: Option<&String>) {
    let mut dir =
        std::env::current_exe().expect("Failed to get executable directory file location!");
    dir.pop();

    #[cfg(unix)]
    dir.push("install.sh");

    #[cfg(windows)]
    dir.push("install.ps1");

    if !dir.exists() {
        std::fs::write(&dir, INSTALL_SCRIPT).expect("Failed to write to install script file!");
    }

    #[cfg(unix)]
    let output = std::process::Command::new("sh")
        .arg(dir)
        .arg(tool_alias(tool_library_name))
        .output()
        .expect("Failed to install tool/library!");

    #[cfg(windows)]
    let output = std::process::Command::new("powershell")
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-Command")
        .arg(dir)
        .arg(tool_alias(aliases(), tool_library_name))
        .output()
        .expect("Failed to install tool/library!");

    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
}

fn tool_alias(tool_name: Option<&String>) -> String {
    let tool = tool_name.unwrap();
    aliases(tool).to_string()
}

#[cfg(unix)]
const fn aliases(tool_name: &str) -> &str {
    match tool_name {
        _ => tool_name,
    }
}

#[cfg(windows)]
fn aliases(tool_name: &str) -> &str {
    match tool_name {
        "gcc" => "mingw",
        _ => tool_name,
    }
    .to_string()
>>>>>>> 8b5c960 (feat: switch from HashMap to match)
}
