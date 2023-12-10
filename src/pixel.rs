//! Character and color management

use crossterm::style::Color;
use std::default;

/// # Style
/// contains boolean data for whether pixels should be bold, italic and underlined
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Style {
    pub bold: bool,
    pub italic: bool,
    pub underlined: bool,
    pub strikethrough: bool,
}
impl default::Default for Style {
    fn default() -> Self {
        Style {
            bold: false,
            italic: false,
            underlined: false,
            strikethrough: false,
        }
    }
}

/// # Pixel structure
/// contains color data and character data
#[derive(Clone, Eq, PartialEq, Copy)]
pub struct Pixel {
    /// Foreground color of the Pixel
    pub fg: Color,
    /// Background color of the Pixel
    pub bg: Color,
    /// Character of the Pixel
    pub chr: char,
    /// Whether the Pixel is bold, italic or underlined
    pub style: Style,
}
impl Pixel {
    /// returns a tuple containing the background and foreground colors of a Pixel
    pub fn get_colors(&self) -> (Color, Color) {
        (self.fg, self.bg)
    }
    pub fn get_style(&self) -> Style {
        self.style
    }
}
impl default::Default for Pixel {
    fn default() -> Self {
        Pixel {
            fg: Color::Reset,
            bg: Color::Reset,
            chr: ' ',
            style: Style::default(),
        }
    }
}

/// Generate a pixel using a character, and optionally forground and background colors.
///
/// usage:
/// ```
/// use console_engine::pixel;
/// // ...
/// engine.set_pxl(0,0,pixel::pxl('X', None, None, None, None, None));
/// ```
pub fn pxl(chr: char, fg: Option<Color>, bg: Option<Color>, style: Option<Style>) -> Pixel {
    Pixel {
        fg: fg.unwrap_or(Color::Reset),
        bg: bg.unwrap_or(Color::Reset),
        chr,
        style: style.unwrap_or(Style::default()),
    }
}

pub fn pxl_plain(char: char) -> Pixel {
    Pixel {
        fg: Color::Reset,
        bg: Color::Reset,
        chr: char,
        style: Style::default(),
    }
}
