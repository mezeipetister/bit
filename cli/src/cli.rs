use crate::cmd::{CommandRegistry, CompletionResult, MatchResult};
use crate::row::Row;
use crate::terminal::Terminal;
use std::borrow::BorrowMut;
use std::env::consts::DLL_SUFFIX;
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
    pub fn exit(&mut self) {
        self.user = None;
        self.project = None;
        self.cwd = "".to_string();
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
        self.terminal.print_str(
            "Welcome to Bit CLI\n-------------------\nType 'help' for a list of commands",
        );
        self.print_start();
    }
    pub fn print_start(&mut self) {
        self.terminal.print_str("To start, type 'open <project>");
    }
    pub fn set_suggestion(&mut self, suggestion: &str) {
        if suggestion.starts_with(&self.context.cwd) {
            let plus = if self.context.cwd.len() > 0 { 1 } else { 0 };
            let start = self.context.cwd.len() + plus;
            self.input = Row::new(&format!("{} ", suggestion[start..].trim()));
        } else {
            self.input = Row::new(suggestion);
        }
    }
    pub fn run(&mut self) -> Result<(), String> {
        self.print_welcome();

        loop {
            // If enter pressed, smart process it
            if self.enter_pressed {
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
                            // Execute the command
                            let res =
                                fn_ptr(&params.join(" "), &mut self.context, &mut self.terminal);

                            // Print the result of the command
                            if let Ok(res) = res {
                                // If result is not empty, print it
                                if res.len() > 0 {
                                    // Print result using terminal print_str
                                    self.terminal.print_str(&res);
                                }
                            } else {
                                // If result is an error, print it
                                println!("Error: {}", res.unwrap_err());
                            }
                        }
                        // Help
                        if let MatchResult::Info(msg) = &cmd_res[0] {
                            self.terminal.goto_first_char();
                            self.terminal.print_str("Help:");
                            self.terminal.print_str(msg);
                        }
                        // If path match, set the cwd
                        if let MatchResult::PathMatch(path) = &cmd_res[0] {
                            self.context.cwd = path.to_string();
                        }
                    }
                    let r = &mut self.input;
                }
                self.enter_pressed = false;
            }
            if self.tab_pressed {
                // Get completions
                let mut completions: Vec<CompletionResult> =
                    self.commands.complete(&self.input.as_str(), &self.context);

                // Dedup completions
                completions.dedup();

                // println!("{:?}", &completions);

                match completions.len() {
                    0 => (),
                    1 => match &completions[0] {
                        CompletionResult::CommandSuggestion(s) => self.set_suggestion(s),
                        CompletionResult::PathSuggestion(s) => self.set_suggestion(s),
                        CompletionResult::None => (),
                    },
                    _ => {
                        self.terminal.goto_first_char();
                        self.terminal.print_str("\nAvailable options:");
                        for c in &completions {
                            match c {
                                CompletionResult::CommandSuggestion(s) => {
                                    self.terminal.print_str(s)
                                }
                                CompletionResult::PathSuggestion(s) => self.terminal.print_str(s),
                                CompletionResult::None => (),
                            }
                        }
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
