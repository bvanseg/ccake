use fansi::{color::AnsiColor, style::AnsiStyle, string::AnsiString};

pub const ANSI_CHOICE_STYLE: [AnsiStyle; 3] = [
    AnsiStyle::Bold,
    AnsiStyle::Underline,
    AnsiStyle::ForegroundColor(AnsiColor::Green),
];
pub const ANSI_ERROR_STYLE: [AnsiStyle; 2] =
    [AnsiStyle::Bold, AnsiStyle::ForegroundColor(AnsiColor::Red)];
pub const ANSI_WARNING_STYLE: [AnsiStyle; 2] =
    [AnsiStyle::Bold, AnsiStyle::ForegroundColor(AnsiColor::Yellow)];

pub fn warning(text: &str) {
    println!("{}", warning_text(text));
}

pub fn warning_text(text: &str) -> String {
    let warning_label = AnsiString::with_styles_arr("warning:", &ANSI_WARNING_STYLE);
    return format!("{} {}", warning_label, text);
}