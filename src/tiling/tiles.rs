use crate::tiling::pane::Pane;
use crate::tiling::text::{TextTile};
use crate::tiling::traits::Printable;
use crate::tiling::progressbar::ProgressBarTile;

pub enum PaneModifier {
    VerticalSplit,
    HorizontalSplit,
    Temporary,
}



pub struct TemporaryTile {
    tile: Box<Tile>,
}

pub enum Tile {
    Line(TextTile),
    ProgressBar(ProgressBarTile),
    Pane(Pane),
    Temporary(TemporaryTile),
}



impl TemporaryTile {
    pub fn create(tile: Tile) -> Tile {
        Tile::Temporary(TemporaryTile {
            tile: Box::from(tile),
        })
    }
}


impl Printable for TemporaryTile {
    fn print(&mut self, pos: (u32, u32), dimensions: (usize, usize)) -> u32 {
        self.tile.print(pos, dimensions)
    }
}
impl Printable for Tile {
    fn print(&mut self, pos: (u32, u32), dimensions: (usize, usize)) -> u32 {
        match self {
            Tile::Line(line) => line.print(pos, dimensions),
            Tile::ProgressBar(progress_bar) => progress_bar.print(pos, dimensions),
            Tile::Pane(pane) => pane.print(pos, dimensions),
            Tile::Temporary(temporary) => temporary.print(pos, dimensions),
        }
    }
}

impl From<TextTile> for Tile {
    fn from(line: TextTile) -> Tile {
        Tile::Line(line)
    }
}
impl From<ProgressBarTile> for Tile {
    fn from(bar: ProgressBarTile) -> Tile {
        Tile::ProgressBar(bar)
    }
}
impl From<Pane> for Tile {
    fn from(pane: Pane) -> Tile {
        Tile::Pane(pane)
    }
}
impl From<TemporaryTile> for Tile {
    fn from(temp: TemporaryTile) -> Tile {
        Tile::Temporary(temp)
    }
}
