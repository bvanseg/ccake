use fansi::{color::AnsiColor, style::AnsiStyle};

pub const ANSI_CHOICE_STYLE: [AnsiStyle; 3] = [
    AnsiStyle::Bold,
    AnsiStyle::Underline,
    AnsiStyle::ForegroundColor(AnsiColor::Green),
];
pub const ANSI_ERROR_STYLE: [AnsiStyle; 2] =
    [AnsiStyle::Bold, AnsiStyle::ForegroundColor(AnsiColor::Red)];
