use std::io::{Stdin, Stdout};

use termion::cursor::DetectCursorPos;
use termion::event::Key;

use crate::row::Row;
use crate::terminal::Terminal;

pub struct Cli<'a, A: FnMut(String) -> Result<(), String>> {
    actions: A,
    should_quit: bool,
    enter_pressed: bool,
    terminal: Terminal<'a>,
    stdin: &'a Stdin,
    input: Row,
    history: Vec<String>,
    history_position: usize,
}

impl<'a, A: FnMut(String) -> Result<(), String>> Cli<'a, A> {
    pub fn new(stdout: &'a Stdout, stdin: &'a Stdin, actions: A) -> Result<Self, std::io::Error> {
        let res = Self {
            actions,
            should_quit: false,
            enter_pressed: false,
            terminal: Terminal::default(stdout)?,
            stdin,
            input: Row::new(""),
            history: Vec::new(),
            history_position: 0,
        };
        Ok(res)
    }
    pub fn run(&mut self) -> Result<(), String> {
        loop {
            if self.enter_pressed {
                // println!("..");
                // println!("{}", self.input.display());
                // println!("{:?}", self.input);
                println!("");
                self.history.push(self.input.as_str().to_string());
                self.history_position = self.history.len();
                self.input = Row::new("");
                let (x, y) = self.terminal._stdout.cursor_pos().unwrap();
                print!("{}", termion::cursor::Goto(1, y));
                (self.actions)(self.history.last().unwrap().to_string()).unwrap();
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
    fn process_keypress(&mut self) -> Result<(), String> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl(c) => self.should_quit = true,
            Key::Char(c) => {
                if c == '\n' {
                    self.enter_pressed = true;
                } else {
                    self.input.insert(c);
                }
            }
            Key::Delete => self.input.delete(),
            Key::Backspace => {
                self.input.backspace();
            }
            Key::Left => self.input.go_left(),
            Key::Right => self.input.go_right(),
            Key::Up => {
                self.input = Row::new(&self.history[self.history_position - 1]);
                if self.history_position > 1 {
                    self.history_position -= 1;
                }
            }
            Key::Down => {
                self.input = Row::new(&self.history[self.history_position - 1]);
                if self.history_position < self.history.len() {
                    self.history_position += 1;
                }
            }
            _ => (),
        }
        Ok(())
    }
    fn render(&mut self) -> Result<(), String> {
        Terminal::cursor_hide();

        let (x, y) = self.terminal._stdout.cursor_pos().unwrap();
        print!("{}", termion::cursor::Goto(1, y));
        print!("{}", self.input.display());

        print!(
            "{}",
            termion::cursor::Goto(self.input.position() as u16 + 1, y)
        );

        Terminal::cursor_show();
        Terminal::blink();
        Terminal::flush().unwrap();

        Ok(())
    }
}

fn die(e: String) {
    Terminal::clear_screen();
    panic!("{}", e);
}
