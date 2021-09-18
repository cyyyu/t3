use crate::term;
use crate::term::Term;
use std::io::stdout;
use std::io::Stdout;
use std::io::Write;
use termion::cursor;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

pub struct Canvas(pub AlternateScreen<RawTerminal<Stdout>>);

impl Canvas {
    pub fn new() -> Self {
        Canvas(AlternateScreen::from(stdout().into_raw_mode().unwrap()))
    }

    pub fn get_size() -> (u16, u16) {
        term::get_size()
    }

    pub fn clear_canvas(&mut self) {
        let stdout = self.get_stdout();
        term::clear_screen(stdout);
        term::goto(stdout, 1, 1);
        term::hide_cursor(stdout);
        self.flush();
    }

    pub fn hide_cursor(&mut self) {
        let stdout = self.get_stdout();
        term::hide_cursor(stdout);
    }

    pub fn dispose(&mut self) {
        let stdout = self.get_stdout();
        term::goto(stdout, 1, 1);
        write!(stdout, "{}", termion::cursor::Show).unwrap();
    }

    pub fn write(&mut self, i: u16, j: u16, c: char) {
        let stdout = self.get_stdout();
        write!(stdout, "{}{}", cursor::Goto(i, j), c).unwrap();
    }

    pub fn flush(&mut self) {
        self.get_stdout().flush().unwrap();
    }

    fn get_stdout(&mut self) -> &mut Term {
        &mut self.0
    }
}
