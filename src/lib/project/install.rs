use crate::lib::constants;
use std::io::Write;

pub fn install(tool_library_name: Option<&String>) {
    let mut dir =
        std::env::current_exe().expect("Failed to get executable directory file location!");
    dir.pop();

    #[cfg(unix)]
    dir.push("install.sh");

    #[cfg(windows)]
    dir.push("install.ps1");

    if !dir.exists() {
        std::fs::write(&dir, constants::INSTALL_SCRIPT)
            .expect("Failed to write to install script file!");
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
        .arg(tool_alias(tool_library_name))
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
    tool_name
    // NOTE: clippy complains when this match statement exists with
    // no arms, but in the future, should we have aliases for linux
    // as well, a match statement should be used
    // match tool_name {
    //     _ => tool_name,
    // }
}

#[cfg(windows)]
const fn aliases(tool_name: &str) -> &str {
    match tool_name {
        "gcc" => "mingw",
        _ => tool_name,
    }
    .to_string()
}
