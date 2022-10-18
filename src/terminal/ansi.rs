use std::fmt;

pub const ANSI_DEFAULT_STYLE: [AnsiStyle; 3] = [AnsiStyle::Bold, AnsiStyle::Underline, AnsiStyle::ForegroundColor(AnsiColor::Green)];
pub const ANSI_CHOICE_STYLE: [AnsiStyle; 3] = [AnsiStyle::Bold, AnsiStyle::Underline, AnsiStyle::ForegroundColor(AnsiColor::Green)];
pub const ANSI_ERROR_STYLE: [AnsiStyle; 2] = [AnsiStyle::Bold, AnsiStyle::ForegroundColor(AnsiColor::Red)];

pub enum AnsiStyle {
    ForegroundColor(AnsiColor),
    Bold,
    Italics,
    Underline
}

impl AnsiStyle {
    const fn code(&self) -> &str {
        match self {
            Self::ForegroundColor(color) => color.code(),
            Self::Bold => "1",
            Self::Italics => "3",
            Self::Underline => "4"
        }
    }
}

pub enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White
}

// ANSI codes generously provided from https://gist.github.com/JBlond/2fea43a3049b38287e5e9cefc87b2124
impl AnsiColor {
    const fn code(&self) -> &'static str {
        match self {
            AnsiColor::Black => "30",
            AnsiColor::Red => "31",
            AnsiColor::Green => "32",
            AnsiColor::Yellow => "33",
            AnsiColor::Blue => "34",
            AnsiColor::Magenta => "35",
            AnsiColor::Cyan => "36",
            AnsiColor::White => "37"
        }
    }
}

pub struct AnsiString {
    pub text: String,
    pub styles: Vec<String>
}

impl AnsiString {
    pub fn from_str(text: &str, styles: Vec<&str>) -> Self {
        AnsiString {
            text: text.to_string(),
            styles: styles.into_iter().map(|f| f.to_string()).collect()
        }
    }

    pub fn from_styles_arr(text: &str, styles: &[AnsiStyle]) -> Self {
        AnsiString {
            text: text.to_string(),
            styles: styles.into_iter().map(|f| f.code().to_string()).collect()
        }
    }

    pub fn from_styles_vec(text: &str, styles: Vec<AnsiStyle>) -> Self {
        AnsiString {
            text: text.to_string(),
            styles: styles.into_iter().map(|f| f.code().to_string()).collect()
        }
    }

    pub fn as_string(&self) -> String {
        format!(
            "\x1b[{}m{}\x1b[{}",
            self.styles.join(";"),
            self.text,
            "0m"
        )
    }
}

impl fmt::Display for AnsiString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.as_string())
    }
}