use std::io::Write;

#[cfg(windows)]
static WINDOWS_INSTALL_SCRIPT: &str = include_str!("../../res/install.ps1");

pub fn install(tool_library_name: Option<&String>) {
    let mut dir = std::env::current_exe().expect("Failed to get executable directory file location!");
    dir.pop();

    #[cfg(unix)]
    dir.push("install.ps1");

    #[cfg(windows)]
    dir.push("install.ps1");

    if !dir.exists() {
        std::fs::write(&dir, WINDOWS_INSTALL_SCRIPT).expect("Failed to write to install script file!");
    }

    let output = std::process::Command::new("powershell")
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-Command")
        .arg(dir)
        .arg(tool_library_name.unwrap())
        .output()
        .expect("Failed to install tool/library!");

    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
}