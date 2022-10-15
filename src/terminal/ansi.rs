use std::fmt;

pub const ANSI_DEFAULT_STYLE: AnsiStyle = AnsiStyle {
    foreground_color: AnsiColor::White,
    bold: false,
    italicize: false,
    underline: false 
};

pub const ANSI_CHOICE_STYLE: AnsiStyle = AnsiStyle {
    foreground_color: AnsiColor::Green,
    bold: true,
    italicize: false,
    underline: true 
};

pub const ANSI_ERROR_STYLE: AnsiStyle = AnsiStyle {
    foreground_color: AnsiColor::Red,
    bold: true,
    italicize: false,
    underline: false 
};

pub enum AnsiColor {
    Black,
    Green,
    Red,
    White
}

// AMSI codes generously provided from https://gist.github.com/JBlond/2fea43a3049b38287e5e9cefc87b2124
impl AnsiColor {
    const fn hex_string(&self) -> &'static str {
        match self {
            AnsiColor::Black => "30m",
            AnsiColor::Red => "31m",
            AnsiColor::Green => "32m",
            AnsiColor::White => "37m"
        }
    }
}

pub struct AnsiStyle {
    pub foreground_color: AnsiColor,
    pub bold: bool,
    pub italicize: bool,
    pub underline: bool
}

impl AnsiStyle {
    pub fn new() -> Self {
        AnsiStyle {
            foreground_color: AnsiColor::White,
            bold: false,
            italicize: false,
            underline: false
        }
    }

    pub fn apply(self, text: &str) -> AnsiString {
        AnsiString::from(text, self)
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn color(mut self, foreground_color: AnsiColor) -> Self {
        self.foreground_color = foreground_color;
        self
    }

    pub fn italicize(mut self) -> Self {
        self.italicize = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }
}

pub struct AnsiString {
    pub text: String,
    pub style: AnsiStyle
}

impl AnsiString {
    pub fn from(text: &str, style: AnsiStyle) -> Self {
        AnsiString {
            text: text.to_string(),
            style: style
        }
    }

    pub fn with_default_style(text: &str) -> Self {
        AnsiString {
            text: text.to_string(),
            style: AnsiStyle {
                foreground_color: AnsiColor::White,
                bold: false,
                italicize: false,
                underline: false
            }
        }
    }

    pub fn as_string(&self) -> String {
        let bold_number = if self.style.bold { 1 } else { 0 };
        let italicize_number = if self.style.italicize { 3 } else { 0 };
        let underline_number = if self.style.underline { 4 } else { 0 };
        let foreground_color_string = self.style.foreground_color.hex_string();
        format!(
            "\x1b[{};{};{};{}{}\x1b[{}",
            bold_number.to_string(),
            italicize_number.to_string(),
            underline_number.to_string(),
            foreground_color_string,
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