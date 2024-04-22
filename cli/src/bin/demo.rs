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
        Command::new("/welcome", "Welcome message", |args, ctx, terminal| {
            Ok("Welcome to Bit CLI".to_string())
        }),
        Command::new("/hi", "Hi", |args, ctx, terminal| Ok("Hi too".to_string())),
        Command::new(
            "/settings/user/print",
            "Print all users",
            |args, ctx, terminal| Ok("Users:...".to_string()),
        ),
        Command::new("/settings/user/add", "Add a user", |args, ctx, terminal| {
            Ok("User added".to_string())
        }),
    ]);
    Cli::new(&stdout, &stdin, commands).unwrap().run().unwrap();
}
