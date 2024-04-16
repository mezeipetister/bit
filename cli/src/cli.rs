use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::io::{Stdin, Stdout};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use termion::cursor::DetectCursorPos;
use termion::event::Key;

use crate::row::Row;
use crate::terminal::Terminal;

#[derive(Default)]
pub struct Context {
    pub project: Option<String>,
    pub cwd: String,
    pub should_quit: bool,
}

pub struct Cli<'a, A: FnMut(String, &'_ mut Context, &'_ mut Terminal) -> Result<String, String>> {
    actions: A,
    enter_pressed: bool,
    terminal: Terminal<'a>,
    stdin: &'a Stdin,
    input: Row,
    history: Vec<String>,
    history_position: usize,
    context: Context,
}

impl<'a, A: FnMut(String, &'_ mut Context, &'_ mut Terminal) -> Result<String, String>> Cli<'a, A> {
    pub fn new(stdout: &'a Stdout, stdin: &'a Stdin, actions: A) -> Result<Self, std::io::Error> {
        let res = Self {
            actions,
            enter_pressed: false,
            terminal: Terminal::default(stdout)?,
            stdin,
            input: Row::new(""),
            history: Vec::new(),
            history_position: 0,
            context: Context::default(),
        };
        Ok(res)
    }
    pub fn print_welcome(&mut self) {
        println!("Welcome to Bit CLI");
        // Set cursor to the first character
        self.terminal.goto_first_char();
        println!("-------------------");
        self.terminal.goto_first_char();
        println!("Type 'help' for a list of commands");
        self.terminal.goto_first_char();
        self.print_start();
    }
    pub fn print_start(&mut self) {
        self.terminal.goto_first_char();
        println!("To start, type 'open <project>'");
        self.terminal.goto_first_char();
    }
    pub fn run(&mut self) -> Result<(), String> {
        self.print_welcome();
        loop {
            if self.enter_pressed {
                // Print start message if project is not set
                // if self.context.project.is_none() {
                //     self.print_start();
                // }

                self.history.push(self.input.as_str().to_string());
                self.history_position = self.history.len();
                self.input = Row::new("");
                let (x, y) = self.terminal._stdout.cursor_pos().unwrap();
                print!("{}", termion::cursor::Goto(1, y));
                {
                    let c = self.context.borrow_mut();
                    let cmd_res = (self.actions)(
                        self.history.last().unwrap().to_string(),
                        &mut self.context, // Specify the lifetime of the borrowed value
                        &mut self.terminal,
                    )
                    .unwrap();
                    if !cmd_res.is_empty() {
                        println!("");
                        println!("{}", cmd_res);
                    }
                    let r = &mut self.input;
                }
                self.enter_pressed = false;
                self.input.set_project(self.context.project.clone());
            }
            if self.context.should_quit {
                self.terminal.goto_first_char();
                break;
            }
            if let Err(error) = self.render() {
                die(error);
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
            Key::Ctrl(c) => self.context.should_quit = true,
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
                if self.history.len() > 0 {
                    if self.history_position > 0 {
                        self.history_position -= 1;
                    }
                    self.input = Row::new(&self.history[self.history_position]);
                }
            }
            Key::Down => {
                if self.history.len() > 0 {
                    if self.history_position < self.history.len() {
                        self.history_position += 1;
                    }
                    self.input = if self.history_position == self.history.len() {
                        Row::new("")
                    } else {
                        Row::new(&self.history[self.history_position])
                    };
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
