use crate::os_impl::TerminalManagerImpl;
use crate::ptui::Ptui;
use crate::tiling::text::{Line, TextTile};
use crate::tiling::traits::Printable;
use crate::traits::{TerminalManager, TextManager};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct ProgressBarTile {
    pub(crate) ui: (char, char, char, char),
    pub(crate) current: Arc<AtomicUsize>,
    pub(crate) total: usize,
    pub(crate) resolution: f32,
    pub(crate) text: Option<(TextTile, usize)>,
}

impl ProgressBarTile {
    pub fn new(
        current: Arc<AtomicUsize>,
        total: usize,
        resolution: usize,
        only_bar: bool,
    ) -> ProgressBarTile {
        let text = Self::generate_text(Arc::clone(&current), total, only_bar);
        let resolution = resolution as f32 / Ptui::get_terminal_size().0 as f32;

        ProgressBarTile {
            ui: ('█', '▒', '│', '│'),
            current,
            total,
            resolution,
            text,
        }
    }
    pub fn new_ex(
        ui: (char, char, char, char),
        current: Arc<AtomicUsize>,
        total: usize,
        resolution: usize,
        only_bar: bool,
    ) -> ProgressBarTile {
        let text = Self::generate_text(Arc::clone(&current), total, only_bar);
        let resolution = resolution as f32 / Ptui::get_terminal_size().0 as f32;

        ProgressBarTile {
            ui,
            current,
            total,
            resolution,
            text,
        }
    }

    fn generate_text(
        current: Arc<AtomicUsize>,
        total: usize,
        only_bar: bool,
    ) -> Option<(TextTile, usize)> {
        if !only_bar {
            let accented_str = Ptui::color_string("Progress:", &Ptui::get_accents());
            let length = accented_str.len();
            let accented_str = Line::convert_static(accented_str);
            let plain_str = vec![Line::Plain(format!(" objects of {}", total))];

            Some((
                TextTile::from_dynamic((accented_str, plain_str), current, Ordering::SeqCst),
                length,
            ))
        } else {
            None
        }
    }

    pub fn incr(&mut self) {
        self.current.fetch_add(1, Ordering::SeqCst);
    }

    fn progress_bar(&mut self, pos: (u32, u32), dimensions: (usize, usize)) -> u32 {
        let text = &mut self.text.as_mut().unwrap().0;

        let row = text.print(pos, dimensions) + 1;
        self.progress_bar_simple((row, pos.1), dimensions)
    }

    fn progress_bar_simple(&self, pos: (u32, u32), dimensions: (usize, usize)) -> u32 {
        Ptui::set_cursor(pos);

        let resolution = self.resolution * dimensions.1 as f32;
        let resolution = resolution as usize;

        let progress_bar_percent = resolution * self.current.load(Ordering::SeqCst) / self.total;

        let (ch, fade, ldelim, rdelim) = self.ui;

        if progress_bar_percent != 100 {
            let fade_str: String = if resolution - progress_bar_percent - 1 > 0 {
                format!(
                    "{fade}{}",
                    " ".repeat(resolution - progress_bar_percent - 1)
                )
            } else {
                fade.to_string()
            };
            print!(
                "{ldelim}{}{fade_str}{rdelim} {}%",
                ch.to_string().repeat(progress_bar_percent),
                progress_bar_percent * 100 / resolution
            );
            return pos.0;
        }

        print!(
            "{ldelim}{}{rdelim} {}%",
            ch.to_string().repeat(progress_bar_percent),
            progress_bar_percent * 100 / resolution
        );

        pos.0
    }
}

impl Printable for ProgressBarTile {
    fn print(&mut self, pos: (u32, u32), dimensions: (usize, usize)) -> u32 {
        Ptui::set_cursor(pos);

        match self.text {
            Some(_) => self.progress_bar(pos, dimensions),
            None => self.progress_bar_simple(pos, dimensions),
        }
    }
}
