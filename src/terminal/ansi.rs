use fansi::{color::AnsiColor, string::AnsiString, style::AnsiStyle};

pub const ANSI_CHOICE_STYLE: [AnsiStyle; 3] = [
    AnsiStyle::Bold,
    AnsiStyle::Underline,
    AnsiStyle::ForegroundColor(AnsiColor::Green),
];
pub const ANSI_ERROR_STYLE: [AnsiStyle; 2] =
    [AnsiStyle::Bold, AnsiStyle::ForegroundColor(AnsiColor::Red)];
pub const ANSI_WARNING_STYLE: [AnsiStyle; 2] = [
    AnsiStyle::Bold,
    AnsiStyle::ForegroundColor(AnsiColor::Yellow),
];

pub fn warning(text: &str) {
    println!("{}", warning_text(text));
}

pub fn error(text: &str) {
    println!("{}", error_text(text));
}

pub fn warning_text(text: &str) -> String {
    label_text("warning:", text, &ANSI_WARNING_STYLE)
}

pub fn error_text(text: &str) -> String {
    label_text("error:", text, &ANSI_ERROR_STYLE)
}

pub fn label_text(label: &str, text: &str, style: &[AnsiStyle]) -> String {
    let label = AnsiString::with_styles_arr(label, style);
    format!("{} {}", label, text)
}
