use crate::modifiers::{Color, TextModifier};
use crate::os_impl::TerminalManagerImpl;
use crate::tiling::pane::Pane;
use crate::tiling::tiles::Tile;
use crate::tiling::traits::Printable;
use crate::traits::{TerminalManager, TextManager};
use std::io::Write;
use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use std::{io, thread};

static PANE: Mutex<Pane> = Mutex::new(Pane::new(0, 0, (0, 1)));
pub(crate) static RUNNING: AtomicBool = AtomicBool::new(true);

pub struct Ptui {
    errors: Vec<String>,
    pane: &'static Mutex<Pane>,
    accents : (Color, Color),
    dimensions: (u16, u16),
}

fn ptui() -> &'static Mutex<Ptui> {
    static PTUI: OnceLock<Mutex<Ptui>> = OnceLock::new();
    PTUI.get_or_init(|| {
        Mutex::new(Ptui {
            errors: vec![],
            pane: &PANE,
            accents: (Color::White, Color::Black),
            dimensions: (0, 0),
        })
    })
}

impl Ptui {
    pub fn init(
        title: String,
        bg: Option<Color>,
        fg: Option<Color>,
        refresh_ms: u64,
    ) {
        let bg = bg.unwrap_or(Color::Black);
        let fg = fg.unwrap_or(Color::White);

        // Enter alternate screen and hide cursor
        print!("\x1B[?1049h\x1B[?25l");
        print!("{}", TextModifier::get_background_modifier(&bg));
        io::stdout().flush().unwrap();
        Self::clear_screen();
        let mut ptui = ptui().lock().unwrap();
        let title = Self::color_string(&title, &fg);
        ptui.pane.lock().unwrap().set_title(title);
        ptui.accents = (fg,bg);

        Ptui::init_signal();

        let _th = thread::spawn(move || {
            while RUNNING.load(Relaxed) {
                Ptui::render();
                thread::sleep(Duration::from_millis(refresh_ms));
            }
        });
        // Allow everything to start properly before returning
        thread::sleep(Duration::from_secs(1));
    }
    pub fn get_bg() -> Color {
        ptui().lock().unwrap().accents.1.clone()
    }
    pub fn get_accents() -> Color {
        ptui().lock().unwrap().accents.0.clone()
    }

    pub fn get_pane() -> &'static Mutex<Pane> {
        &PANE
    }
    pub fn push(tile: Tile) -> usize {
        PANE.lock().unwrap().push(tile)
    }

    fn render_loop(&mut self) {
        let (rows, cols) = Self::get_terminal_size();
        if (rows, cols) != self.dimensions {
            Self::clear_screen();
            self.dimensions = (rows, cols);
        }

        let mut pane = self.pane.lock().expect("Unable to lock pane");
        pane.print((3, 3), (rows as usize - 3, cols as usize - 3));
        io::stdout().flush().unwrap();
    }
    fn render() {
        ptui().lock().unwrap().render_loop();
    }

    pub fn finalize() {
        Self::clear_screen();
        print!("\x1B[0m"); // Reset background and foreground
        print!("\x1B[?25h"); // Restore cursor
        print!("\x1B[?1049l"); // exit alternate screen

        io::stdout().flush().unwrap();
        let ptui = ptui().lock().expect("Unable to lock ptui");

        for error in ptui.errors.iter() {
            println!("{}", error);
        }
        exit(0)
    }
}
impl TerminalManager for Ptui {}
impl TerminalManagerImpl for Ptui {}
impl TextManager for Ptui {}

impl Drop for Ptui {
    fn drop(&mut self) {
        Ptui::finalize();
    }
}

#[macro_export]
macro_rules! ptui_pushln { ($($arg:tt)*) => { Ptui::pushln(format_args!($($arg)*)) }; }
#[macro_export]
macro_rules! ptui_push { ($($arg:tt)*) => { Ptui::push(format_args!($($arg)*)); };}
