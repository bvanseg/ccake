use super::prompt;
use super::ansi;

pub fn create_new_project() {
    let ansi_project_name = ansi::ANSI_CHOICE_STYLE.apply("Project Name:");
    let ansi_project_version = ansi::ANSI_CHOICE_STYLE.apply("Version:");

    let project_name = prompt(ansi_project_name);
    let project_version = prompt(ansi_project_version);

    // TODO: Generate TOML file.
}