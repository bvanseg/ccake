use crate::lib::constants;
use std::collections::HashMap;
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
        .arg(tool_alias(aliases(), tool_library_name))
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

fn tool_alias(aliases: HashMap<String, String>, tool_name: Option<&String>) -> String {
    let tool = tool_name.unwrap();
    let alias = aliases.get(tool);
    alias.unwrap_or(tool).to_owned()
}

#[cfg(unix)]
fn aliases() -> HashMap<String, String> {
    HashMap::new()
}

#[cfg(windows)]
fn aliases() -> HashMap<String, String> {
    let mut aliases = HashMap::new();
    aliases.insert("gcc".to_string(), "mingw".to_string());
    aliases
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tool_alias_with_alias() {
        let mut aliases = HashMap::new();
        aliases.insert("gcc".to_string(), "mingw".to_string());
        assert_eq!(
            tool_alias(aliases, Some(&"gcc".to_string())),
            "mingw".to_string()
        );
    }
    #[test]
    fn tool_alias_with_no_alias() {
        let aliases = HashMap::new();
        assert_eq!(
            tool_alias(aliases, Some(&"gcc".to_string())),
            "gcc".to_string()
        );
    }
}
