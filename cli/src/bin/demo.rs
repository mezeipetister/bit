use std::{cell::RefCell, rc::Rc};

use cli::{
    cli::Context,
    cmd::{Command, CommandRegistry},
    terminal::Terminal,
    Cli,
};

fn main() {
    let stdout = std::io::stdout();
    let stdin = std::io::stdin();
    let commands = CommandRegistry::new(vec![
        Command::new("/welcome", "Welcome message", |arg| {
            Ok("Welcome to Bit CLI".to_string())
        }),
        Command::new("/hi", "Hi", |arg| Ok("Hi too".to_string())),
        Command::new("/settings/user/print", "Print all users", |arg| {
            Ok("Users:...".to_string())
        }),
    ]);
    Cli::new(&stdout, &stdin, commands).unwrap().run().unwrap();
}
