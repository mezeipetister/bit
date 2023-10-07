use std::io::stdout;

use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::MouseTerminal;

use crate::row::Row;
use crate::terminal::Terminal;

pub struct Cli {
    should_quit: bool,
    enter_pressed: bool,
    terminal: Terminal,
    input: Row,
    input_history: Vec<String>,
    cursor_pos_x: usize,
    path: String,
    user: String,
    repo: String,
}

impl Cli {
    pub fn new() -> Result<Self, std::io::Error> {
        let res = Self {
            should_quit: false,
            enter_pressed: false,
            terminal: Terminal::default()?,
            input: Row::from(""),
            input_history: Vec::new(),
            cursor_pos_x: 0,
            path: String::new(),
            user: "mezeipeti".to_string(),
            repo: "gz".to_string(),
        };
        Ok(res)
    }
    pub fn run(&mut self) -> Result<(), String> {
        loop {
            if self.enter_pressed {
                print!("{}", '\n');
                self.input = Row::default();
                self.cursor_pos_x = 0;
                self.enter_pressed = false;
            }
            if let Err(error) = self.render() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }

        Ok(())
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl(c) => self.should_quit = true,
            Key::Char(c) => {
                if c == '\n' {
                    self.enter_pressed = true;
                } else {
                    self.input.insert(self.cursor_pos_x, c);
                    self.move_cursor(Key::Right);
                }
            }
            Key::Delete => self.input.delete(self.cursor_pos_x),
            Key::Backspace => {
                if self.cursor_pos_x > 0 {
                    self.move_cursor(Key::Left);
                    self.input.delete(self.cursor_pos_x);
                }
            }
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }
    fn move_cursor(&mut self, key: Key) {
        let mut x = self.cursor_pos_x;
        let mut width = self.input.len();

        match key {
            Key::Up => (),
            Key::Down => (),
            Key::Left => {
                if x > 0 {
                    x -= 1;
                }
            }
            Key::Right => {
                if x < width {
                    x += 1;
                }
            }
            _ => (),
        }
        if x > width {
            x = width;
        }

        self.cursor_pos_x = x;
    }
    fn render(&mut self) -> Result<(), std::io::Error> {
        Terminal::clear_current_line();
        Terminal::flush()?;
        let (x, y) = self.terminal._stdout.cursor_pos()?;
        print!("{}", termion::cursor::Goto(1, y));
        print!("{}-{}: {}", &self.user, &self.repo, self.input.as_str());
        print!("{}", termion::cursor::Goto(self.cursor_pos_x as u16, y));
        Terminal::cursor_show();
        Terminal::flush()?;
        Ok(())
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
