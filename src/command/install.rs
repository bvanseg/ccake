use std::io::Write;

#[cfg(unix)]
static UNIX_INSTALL_SCRIPT: &str = include_str!("../../res/install.sh");

#[cfg(windows)]
static WINDOWS_INSTALL_SCRIPT: &str = include_str!("../../res/install.ps1");

pub fn install(tool_library_name: Option<&String>) {
    let mut dir =
        std::env::current_exe().expect("Failed to get executable directory file location!");
    dir.pop();

    #[cfg(unix)]
    dir.push("install.sh");

    #[cfg(windows)]
    dir.push("install.ps1");

    if !dir.exists() {
        #[cfg(unix)]
        std::fs::write(&dir, UNIX_INSTALL_SCRIPT).expect("Failed to write to install script file!");

        #[cfg(windows)]
        std::fs::write(&dir, WINDOWS_INSTALL_SCRIPT)
            .expect("Failed to write to install script file!");
    }

    #[cfg(windows)]
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
