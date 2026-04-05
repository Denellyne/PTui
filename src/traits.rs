use crate::modifiers::Color::White;
pub(crate) use crate::modifiers::{Color, TextModifier};
use crate::os_impl::TerminalManagerImpl;
use crate::ptui::Ptui;
use crate::tiling::text::TextTile;
use crate::tiling::tiles::Tile;
use std::io::{Read, Write, stdin, stdout};
use crate::modifiers::Color::Black;

pub(crate) trait TerminalManager: TerminalManagerImpl {
    fn clear_screen()
    where
        Self: Sized,
    {
        print!("\x1B[2J\x1B[1;1H");
        stdout().flush().unwrap();
    }

    fn set_cursor(pos: (u32, u32))
    where
        Self: Sized,
    {
        print!("\x1B[{};{}H", pos.0 + 1, pos.1 + 1);
        stdout().flush().unwrap();
    }
    // fn reset_cursor()
    // where
    //     Self: Sized,
    // {
    //     print!("\x1B[H");
    //     stdout().flush().unwrap();
    // }
    // fn clear_line() -> String
    // where
    //     Self: Sized,
    // {
    //     "\x1B[1A\x1B[K".to_string()
    // }
}

pub trait TextManager {
    fn color_string(text: &str, modifier: &Color) -> String {
        let modifier = TextModifier::get_foreground_modifier(modifier);
        let default = TextModifier::get_foreground_modifier(&White);
        format!("{modifier}{text}{default}")
    }

    fn color_string_ex(
        text: String,
        modifier: Color,
        default: Color,
    ) -> String {
        let modifier = TextModifier::get_foreground_modifier(&modifier);
        let default = TextModifier::get_foreground_modifier(&default);
        format!("{modifier}{text}{default}")
    }

    fn wait_input() {
        let _ = Ptui::push(Tile::Line(TextTile::from_plain(
            "Press any key to exit.",
        )));
        let _ = stdin().read(&mut [0u8]).unwrap();
    }

    fn set_foreground(foreground: Color) {
        print!("{}", TextModifier::get_foreground_modifier(&foreground));
    }

    fn reset_foreground() {
        print!("{}", TextModifier::get_foreground_modifier(&White));
    }
    fn reset_background() {
        print!("{}", TextModifier::get_background_modifier(&Ptui::get_bg()));
    }
    fn set_background(background: Color) {
        print!("{}", TextModifier::get_background_modifier(&background));
    }
}
pub trait Colored {
    fn foreground(&self,fg: &Color) -> String where Self: std::fmt::Display {
        let fg = TextModifier::get_foreground_modifier(fg);
        let default = TextModifier::get_foreground_modifier(&White);
        format!("{fg}{self}{default}")
    }

    fn to_background(&self,bg: &Color) -> String  where Self: std::fmt::Display{
        let bg = TextModifier::get_background_modifier(bg);
        let default = TextModifier::get_background_modifier(&Black);
        format!("{bg}{self}{default}")
    }

    fn black(&self) -> String where Self: std::fmt::Display {
        let fg = TextModifier::get_foreground_modifier(&Black);
        let default = TextModifier::get_foreground_modifier(&White);
        format!("{fg}{self}{default}")
    }

    fn red(&self) -> String where Self: std::fmt::Display {
        let fg = TextModifier::get_foreground_modifier(&Color::Red);
        let default = TextModifier::get_foreground_modifier(&White);
        format!("{fg}{self}{default}")
    }

    fn green(&self) -> String where Self: std::fmt::Display {
        let fg = TextModifier::get_foreground_modifier(&Color::Green);
        let default = TextModifier::get_foreground_modifier(&White);
        format!("{fg}{self}{default}")
    }

    fn blue(&self) -> String  where Self: std::fmt::Display{
        let fg = TextModifier::get_foreground_modifier(&Color::Blue);
        let default = TextModifier::get_foreground_modifier(&White);
        format!("{fg}{self}{default}")
    }

    fn yellow(&self) -> String  where Self: std::fmt::Display{
        let fg = TextModifier::get_background_modifier(&Color::Yellow);
        let default = TextModifier::get_foreground_modifier(&White);
        format!("{fg}{self}{default}")
    }

    fn magenta(&self) -> String  where Self: std::fmt::Display{
        let fg = TextModifier::get_background_modifier(&Color::Magenta);
        let default = TextModifier::get_foreground_modifier(&White);
        format!("{fg}{self}{default}")
    }

    fn cyan(&self) -> String  where Self: std::fmt::Display{
        let fg = TextModifier::get_background_modifier(&Color::Cyan);
        let default = TextModifier::get_foreground_modifier(&White);
        format!("{fg}{self}{default}")
    }

    fn to_white(&self) -> String  where Self: std::fmt::Display{
        let bg = TextModifier::get_background_modifier(&White);
        let default = TextModifier::get_background_modifier(&Black);
        format!("{bg}{self}{default}")
    }

    fn to_red(&self) -> String  where Self: std::fmt::Display{
        let bg = TextModifier::get_background_modifier(&Color::Red);
        let default = TextModifier::get_background_modifier(&Black);
        format!("{bg}{self}{default}")
    }

    fn to_green(&self) -> String  where Self: std::fmt::Display{
        let bg = TextModifier::get_background_modifier(&Color::Green);
        let default = TextModifier::get_background_modifier(&Black);
        format!("{bg}{self}{default}")
    }

    fn to_blue(&self) -> String  where Self: std::fmt::Display{
        let bg = TextModifier::get_background_modifier(&Color::Blue);
        let default = TextModifier::get_background_modifier(&Black);
        format!("{bg}{self}{default}")
    }

    fn to_yellow(&self) -> String  where Self: std::fmt::Display{
        let bg = TextModifier::get_background_modifier(&Color::Yellow);
        let default = TextModifier::get_background_modifier(&Black);
        format!("{bg}{self}{default}")
    }

    fn to_magenta(&self) -> String  where Self: std::fmt::Display{
        let bg = TextModifier::get_background_modifier(&Color::Magenta);
        let default = TextModifier::get_background_modifier(&White);
        format!("{bg}{self}{default}")
    }

    fn to_cyan(&self) -> String  where Self: std::fmt::Display{
        let bg = TextModifier::get_background_modifier(&Color::Cyan);
        let default = TextModifier::get_background_modifier(&White);
        format!("{bg}{self}{default}")
    }

    fn ptui_bg(&self) -> String  where Self: std::fmt::Display{
        let bg = Ptui::get_bg();
        let bg = TextModifier::get_background_modifier(&bg);
        let default = TextModifier::get_background_modifier(&White);
        format!("{bg}{self}{default}")
    }
    fn ptui_fg(&self) -> String  where Self: std::fmt::Display{
        let fg =Ptui::get_accents();
        let fg = TextModifier::get_foreground_modifier(&fg);
        let default = TextModifier::get_foreground_modifier(&White);
        format!("{fg}{self}{default}")
    }
}

impl Colored for String {

}

impl Colored for &'static str {

}
