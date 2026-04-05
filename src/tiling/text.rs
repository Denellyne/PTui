use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::modifiers::{ForegroundModifier, TextModifier};
use crate::tiling::pane::Pane;
use crate::tiling::traits::Printable;
use crate::traits::{TerminalManager, TextManager};

#[derive(Clone,Debug)]
pub enum CustomLine{
    Static(String, ForegroundModifier),
    Dynamic(Arc<AtomicUsize>, Ordering),
}
#[derive(Clone,Debug)]
pub enum Line {
    Plain(String),
    Custom(CustomLine),
}
#[derive(Clone,Debug)]
pub struct TextTile {
    lines: Vec<Line>,
}


impl TextTile {
    pub fn from_plain(text : String) -> Self {
        TextTile{lines: vec![Line::Plain(text)]}
    }

    pub fn from_static(text : String) -> Self {
        TextTile{lines: Line::convert_static(text)}
    }
    pub fn from_dynamic(text : (Vec<Line>,Vec<Line>),atomic :Arc<AtomicUsize>, ord : Ordering) -> Self {
        TextTile{lines: Line::convert_dynamic(text,atomic, ord)}
    }



    fn print_modifier(&self, modifier: &ForegroundModifier) {
        print!("{}", TextModifier::get_foreground_modifier(modifier));
    }
}
impl Line{
   pub fn convert_static(mut string: String) -> Vec<Line> {
        let mut lines: Vec<Line> = vec![];

        while let Some(val) = string.find("\x1B[") {
            if val > 0 {
                lines.push(Line::Plain(string.drain(..val).collect()));
            }
            let modifier: String = string
              .drain(..=string.find("m").expect("Malformed Opening Modifier"))
              .collect();
            let str: String = string
              .drain(..string.find("\x1B[").expect("Malformed Closing Modifier"))
              .collect();
            lines.push(Line::Custom(CustomLine::Static(str, ForegroundModifier::Custom(modifier))));
            let _: String = string
              .drain(..=string.find("m").expect("Malformed Closing Modifier"))
              .collect();
        }

        if !string.is_empty() {
            lines.push(Line::Plain(string));
        }
        lines
    }

   pub fn convert_dynamic(text : (Vec<Line>,Vec<Line>), atomic :Arc<AtomicUsize>, ord : Ordering) -> Vec<Line> {
        let (pre,sub) = text;
        let dynamic = vec![Line::Custom(CustomLine::Dynamic(atomic, ord))];
        let lines: Vec<Line> = vec![pre,dynamic,sub].into_iter().flatten().collect();

        lines
    }
}

impl Printable for TextTile {
    fn print(&mut self, pos: (u32, u32), dimensions: (usize, usize)) -> u32 {
        let (mut rows, cols) = pos;
        let (height, width) = dimensions;
        let mut buf = String::with_capacity(128);

        for line in &self.lines {
            if width == 0 || height.saturating_sub(rows as usize) == 0 {
                break;
            }
            let is_custom = match line {
                Line::Plain(slice) => {
                    buf.clear();
                    buf.push_str(slice);
                    false
                }
                Line::Custom(c) => match c {
                    CustomLine::Static(str, foreground) => {
                        self.print_modifier(foreground);
                        buf.clear();
                        buf.push_str(str);
                        true
                    }
                    CustomLine::Dynamic(atomic,ord) =>{
                        buf = format!("{}",  atomic.load(*ord));
                        true
                    }

                }
            };
            let mut slice = buf.as_str();

            let mut length = slice.len();

            while length >= width {
                Pane::set_cursor((rows, cols));
                let str = &slice[..width];

                print!("{str}");
                rows += 1;
                length -= width;
                slice = &slice[width..];
            }

            if !slice.is_empty() {
                print!("{slice}");
            }
            if is_custom {
                print! {"{}",TextModifier::get_foreground_modifier(&ForegroundModifier::White)}
            }
        }

        rows
    }
}
impl TextManager for TextTile {}
