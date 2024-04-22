use cli::{
    cmd::{Command, CommandRegistry},
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
        Command::new("exit", "Close BIT", |_, ctx, _| {
            ctx.should_quit = true;
            Ok("bye".to_string())
        }),
        Command::new("help", "Help", |_, _, _| {
            Ok("Available commands:\n/open, /create, exit, help\n".to_string())
        }),
        Command::new("count", "Count", |_, _, _| {
            Ok((0..100)
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join("\n"))
        }),
    ];

    let commands = vec![
        Command::new("/welcome", "Welcome message", |_, _, _| {
            Ok("Welcome to Bit CLI".to_string())
        }),
        Command::new("/hi", "Hi", |_, _, _| Ok("Hi too".to_string())),
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
        Command::new("exit", "Exit project", |_, ctx, _| {
            let pname = ctx.project.to_owned().unwrap_or("".to_string());
            ctx.exit();
            Ok(format!("Logout from project: {}", pname))
        }),
    ];

    let commands = CommandRegistry::new(pre, commands);
    Cli::new(&stdout, &stdin, commands).unwrap().run().unwrap();
}
