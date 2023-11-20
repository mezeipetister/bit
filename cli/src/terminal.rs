use std::io::{self, stdout, Write};
use termion::color;
use termion::cursor::DetectCursorPos;
use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};

#[derive(Default, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Terminal<'a> {
    pub _stdout: RawTerminal<&'a std::io::Stdout>,
}

impl<'a> Terminal<'a> {
    pub fn default(stdout: &'a std::io::Stdout) -> Result<Self, std::io::Error> {
        let mut res = Self {
            _stdout: stdout.into_raw_mode()?,
        };
        Ok(res)
    }
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }
    pub fn read_key() -> Result<Key, String> {
        if let Some(key) = io::stdin().lock().keys().next() {
            return Ok(key.unwrap());
        }
        Err("Oooo".into())
    }
    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }
    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }
    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }
    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }
    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }
    pub fn go_left() {
        print!("{}", termion::cursor::Left(1));
    }
    pub fn go_right() {
        print!("{}", termion::cursor::Right(1));
    }
    pub fn blink() {
        print!("{}", termion::cursor::BlinkingBar);
    }
    pub fn backspace(&mut self) {
        let (x, y) = self._stdout.cursor_pos().unwrap();
        write!(self._stdout, "{}{}", " ", termion::cursor::Goto(x - 1, y));
    }
}
