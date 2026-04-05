use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Color{
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Custom(String),
}


#[derive(Debug, Clone)]
pub enum TextModifier {
    Foreground(Color),
    Background(Color),
}

impl TextModifier {
    pub fn get(modifier: &TextModifier) -> &str {
        match modifier {
            TextModifier::Foreground(modifier) => Self::get_foreground_modifier(modifier),
            TextModifier::Background(modifier) => Self::get_background_modifier(modifier),
        }
    }
    pub fn get_background_modifier(modifier: &Color) -> &str {
        match modifier {
            Color::Black => "\x1B[40m",
            Color::Red => "\x1B[41m",
            Color::Green => "\x1B[42m",
            Color::Yellow => "\x1B[43m",
            Color::Blue => "\x1B[44m",
            Color::Magenta => "\x1B[45m",
            Color::Cyan => "\x1B[46m",
            Color::White => "\x1B[47m",
            Color::Custom(s) => s,
        }
    }
    pub fn get_foreground_modifier(modifier: &Color) -> &str {
        match modifier {
            Color::Black => "\x1B[30m",
            Color::Red => "\x1B[31m",
            Color::Green => "\x1B[32m",
            Color::Yellow => "\x1B[33m",
            Color::Blue => "\x1B[34m",
            Color::Magenta => "\x1B[35m",
            Color::Cyan => "\x1B[36m",
            Color::White => "\x1B[37m",
            Color::Custom(s) => s.as_ref(),
        }
    }
}



impl Display for TextModifier {
    fn fmt(&self, b: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = TextModifier::get(self);
        write!(b, "{}", val)
    }
}
