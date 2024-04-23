use std::arch::global_asm;

use crate::{cli::Context, terminal::Terminal};

#[derive(Debug, PartialEq)]
pub enum MatchResult {
    CommandMatch(
        Vec<String>,
        fn(&str, &mut Context, &mut Terminal) -> Result<String, String>,
    ),
    CommandSuggestion(String),
    PathMatch(String),
    PathSuggestion(String),
    None,
}

pub struct CommandRegistry {
    pre_commands: Vec<Command>,
    commands: Vec<Command>,
}

impl CommandRegistry {
    pub fn new(pre_commands: Vec<Command>, commands: Vec<Command>) -> Self {
        Self {
            pre_commands,
            commands,
        }
    }
    pub fn run(&self, input: &str, ctx: &Context) -> Vec<MatchResult> {
        let mut results = Vec::new();

        if ctx.project.is_some() {
            for cmd in &self.commands {
                let result = cmd.match_path(input, ctx);
                if result != MatchResult::None {
                    results.push(result);
                }
            }
        } else {
            for cmd in &self.pre_commands {
                let result = cmd.match_path(input, ctx);
                if result != MatchResult::None {
                    results.push(result);
                }
            }
        }

        results
    }
}

pub struct Command {
    path: &'static str,
    help: &'static str,
    fn_ptr: fn(&str, &mut Context, &mut Terminal) -> Result<String, String>,
}

impl Command {
    pub fn new(
        path: &'static str,
        help: &'static str,
        fn_ptr: fn(&str, &mut Context, &mut Terminal) -> Result<String, String>,
    ) -> Self {
        Self { path, fn_ptr, help }
    }

    pub fn match_path(&self, raw_input: &str, ctx: &Context) -> MatchResult {
        let is_global = !self.path.starts_with("/");

        let command_tokens = tokenize_line(self.path);

        let mut input = raw_input.trim().to_string();

        let input_tokens = tokenize_line(&input);

        // Check global command match
        if is_global {
            if input.starts_with(&self.path) {
                return MatchResult::CommandMatch(
                    input_tokens[command_tokens.len()..].to_vec(),
                    self.fn_ptr,
                );
            }
        }

        if !input.starts_with("/") {
            input = format!("/{}/{}", &ctx.cwd, input);
        }

        let input_tokens = tokenize_line(&input);

        // Check complete command match
        if input_tokens.starts_with(&command_tokens) {
            return MatchResult::CommandMatch(
                input_tokens[command_tokens.len()..].to_vec(),
                self.fn_ptr,
            );
        }

        // Suggesting command
        let (command_path, command_cmd) = split_last_token(&command_tokens);
        let (input_path, input_cmd) = split_last_token(&input_tokens);

        if command_path == input_path {
            if let Some(cmd) = command_cmd {
                if let Some(_cmd) = input_cmd {
                    if cmd.starts_with(_cmd) {
                        return MatchResult::CommandSuggestion(match command_path.len() {
                            0 => format!("/{}", command_cmd.unwrap()),
                            _ => format!("/{} {}", command_path.join("/"), command_cmd.unwrap()),
                        });
                    }
                }
            }
        }

        // Path match
        if command_path
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .starts_with(&input_tokens)
        {
            let command_path = vec!["".to_string()]
                .into_iter()
                .chain(input_tokens.into_iter())
                .collect::<Vec<String>>();
            return MatchResult::PathMatch(command_path.join("/"));
        }

        // Path suggestion
        if self.path.starts_with(&input) {
            if let Some(p) = command_path.last() {
                return MatchResult::PathSuggestion(format!("/{}", command_path.join("/")));
            }
        }

        MatchResult::None
    }
}

fn split_last_token(tokens: &Vec<String>) -> (Vec<&str>, Option<&String>) {
    if tokens.is_empty() {
        return (vec![], None);
    }

    if tokens.len() > 1 {
        let last = tokens.last().unwrap();
        let rest = &tokens[..tokens.len() - 1];
        return (rest.iter().map(|s| s.as_str()).collect(), Some(last));
    }

    (vec![], tokens.get(0))
}

fn tokenize_line(line: &str) -> Vec<String> {
    let mut tokens = vec![];
    let mut current_token = String::new();
    let mut inside_quotes = false;

    for c in line.chars() {
        if c == '"' {
            inside_quotes = !inside_quotes;
            current_token.push(c);
        } else if c == '/' && !inside_quotes {
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
        } else if c.is_whitespace() && !inside_quotes {
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
        } else {
            current_token.push(c);
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

// fn main() {
//     let cmd = Command::new(
//         "/settings/user print",
//         "Print user settings lorem ipsum dolorem set ami",
//         |_| Err("".to_string()),
//     );

//     println!("{:?}", cmd.match_path("/settings/user print"));
//     println!("{:?}", cmd.match_path("/settings/user print all"));
//     println!("{:?}", cmd.match_path("/settings/user print all params"));
//     println!("{:?}", cmd.match_path("/settings/user print all a=1 b=2"));
//     println!("{:?}", cmd.match_path("/settings/user pri"));
//     println!("{:?}", cmd.match_path("/settings/user"));
//     println!("{:?}", cmd.match_path("/settings/us"));

//     let cmd = Command::new("/note create", "Create a new note", |_| Err("".to_string()));

//     println!("{:?}", cmd.match_path("/note create new"));
//     println!("{:?}", cmd.match_path("/note create"));
//     println!("{:?}", cmd.match_path("/note cr"));
//     println!("{:?}", cmd.match_path("/note"));
//     println!("{:?}", cmd.match_path("/no"));

//     // println!("{:?}", &r);
// }
