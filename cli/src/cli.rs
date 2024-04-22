use crate::cmd::{CommandRegistry, MatchResult};
use crate::row::Row;
use crate::terminal::Terminal;
use std::borrow::BorrowMut;
use std::io::{Stdin, Stdout};
use termion::cursor::DetectCursorPos;
use termion::event::Key;

#[derive(Default)]
pub struct Context {
    pub user: Option<String>,
    pub project: Option<String>,
    pub cwd: String,
    pub should_quit: bool,
}

impl Context {
    pub fn set_cwd(&mut self, path: String) {
        if path.len() == 0 {
            let path = "/".to_string();
        }
        if !path.starts_with("/") {
            let path = format!("/{}", path);
        }
        self.cwd = path;
    }
}

pub struct Cli<'a> {
    commands: CommandRegistry,
    enter_pressed: bool,
    tab_pressed: bool,
    terminal: Terminal<'a>,
    stdin: &'a Stdin,
    input: Row,
    history: Vec<String>,
    history_position: usize,
    context: Context,
}

impl<'a> Cli<'a> {
    pub fn new(
        stdout: &'a Stdout,
        stdin: &'a Stdin,
        commands: CommandRegistry,
    ) -> Result<Self, std::io::Error> {
        let res = Self {
            commands,
            enter_pressed: false,
            tab_pressed: false,
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
                    let cmd_res = self
                        .commands
                        .run(&self.history.last().unwrap(), &self.context);

                    // Print empty line to separate the command from the result
                    println!("");

                    // println!("{:?}", &cmd_res);

                    if cmd_res.is_empty() {
                        println!("Unknown command");
                    } else {
                        if let MatchResult::CommandMatch(params, fn_ptr) = &cmd_res[0] {
                            let res =
                                fn_ptr(&params.join(" "), &mut self.context, &mut self.terminal);
                            if let Ok(res) = res {
                                println!("{}", res);
                            } else {
                                println!("Error: {}", res.unwrap_err());
                            }
                        }
                        if let MatchResult::PathMatch(path) = &cmd_res[0] {
                            self.context.cwd = path.to_string();
                        }
                    }
                    let r = &mut self.input;
                }
                self.enter_pressed = false;
            }
            if self.tab_pressed {
                let completions: Vec<MatchResult> = self
                    .commands
                    .run(&self.input.as_str(), &self.context)
                    .into_iter()
                    .filter(|r| match r {
                        MatchResult::CommandSuggestion(_) => true,
                        MatchResult::PathSuggestion(_) => true,
                        _ => false,
                    })
                    .collect();
                // println!("{:?}", &completions);
                if let Some(c) = completions.first() {
                    match c {
                        MatchResult::CommandSuggestion(s) => {
                            self.input = Row::new(s);
                        }
                        MatchResult::PathSuggestion(s) => {
                            self.input = Row::new(s);
                        }
                        _ => (),
                    }
                }
                self.tab_pressed = false;
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
                } else if c == '\t' {
                    self.tab_pressed = true;
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
        // print!("{}", self.input.display());

        print!("{}", self.input.display(&self.context));

        print!(
            "{}",
            termion::cursor::Goto(self.input.position(&self.context) as u16 + 1, y)
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
