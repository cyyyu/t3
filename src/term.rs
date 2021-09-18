use std::io::{Stdout, Write};
use termion::cursor::Goto;
use termion::raw::RawTerminal;

pub type Term = RawTerminal<Stdout>;

pub fn clear_screen(stdout: &mut Term) {
    write!(stdout, "{}", termion::clear::All).unwrap();
}

pub fn goto(stdout: &mut Term, x: u16, y: u16) {
    write!(stdout, "{}", Goto(x, y)).unwrap();
}

pub fn hide_cursor(stdout: &mut Term) {
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
}

pub fn get_size() -> (u16, u16) {
    termion::terminal_size().unwrap()
}
