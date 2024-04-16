use std::{cell::RefCell, rc::Rc};

use cli::{cli::Context, terminal::Terminal, Cli};

fn main() {
    let stdout = std::io::stdout();
    let stdin = std::io::stdin();
    let actions =
        |i: String, ctx: &mut Context, terminal: &mut Terminal| -> Result<String, String> {
            let tokens = cli::input_tokens::parse_input(&i);

            if tokens.cmd().is_none() {
                return Ok("".into());
            }

            match tokens.cmd().unwrap() {
                "hello" => Ok("Bello".into()),
                "yuhuu" => Ok("yuhuu too".into()),
                "clear" => {
                    // Clear terminal screen
                    print!("\x1B[2J\x1B[1;1H");
                    Ok("".into())
                }
                "exit" => match &ctx.project {
                    Some(p) => {
                        ctx.project = None;
                        Ok("exiting project".into())
                    }
                    None => {
                        ctx.should_quit = true;
                        Ok("bye".into())
                    }
                },
                "open" => {
                    ctx.project = tokens.get(1).map(|s| s.to_string());
                    Ok("opening project".into())
                }
                "quit" => {
                    ctx.should_quit = true;
                    Ok("bye".into())
                }
                _ => Ok("unknown command".into()),
            }
        };
    Cli::new(&stdout, &stdin, actions).unwrap().run().unwrap();
}
