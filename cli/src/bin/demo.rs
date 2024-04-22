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

    let pre = vec![
        Command::new("/open", "Open project", |args, ctx, terminal| {
            if args.is_empty() {
                return Err("Please provide a name".to_string());
            }
            ctx.project = Some(args.to_string());
            Ok("Opening project".to_string())
        }),
        Command::new("/create", "Creating project", |args, ctx, terminal| {
            Ok("Creating project".to_string())
        }),
    ];

    let commands = vec![
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
        Command::new("..", "Step back in path", |args, ctx, terminal| {
            // Remove ctx.cwd last element
            let mut path = ctx.cwd.split('/').collect::<Vec<&str>>();
            path.pop();
            ctx.cwd = path.join("/");
            Ok("".to_string())
        }),
    ];

    let commands = CommandRegistry::new(pre, commands);
    Cli::new(&stdout, &stdin, commands).unwrap().run().unwrap();
}
