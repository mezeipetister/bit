#[derive(Debug)]
enum MatchResult {
    CommandMatch(Vec<String>, fn(&str) -> Result<String, String>),
    CommandSuggestion(String),
    PathMatch(String),
    PathSuggestion(String),
    None,
}

struct Command {
    path: &'static str,
    help: &'static str,
    fn_ptr: fn(&str) -> Result<String, String>,
}

impl Command {
    fn new(
        path: &'static str,
        help: &'static str,
        fn_ptr: fn(&str) -> Result<String, String>,
    ) -> Self {
        Self { path, fn_ptr, help }
    }

    fn match_path(&self, path: &str) -> MatchResult {
        let command_tokens = tokenize_line(self.path);
        let input_tokens = tokenize_line(path);
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
                        return MatchResult::CommandSuggestion(cmd.to_string());
                    }
                }
            }
        }

        // Path match
        if command_path == input_tokens {
            return MatchResult::PathMatch(path.to_string());
        }

        // Path suggestion
        if self.path.starts_with(&path) {
            if let Some(p) = command_path.last() {
                return MatchResult::PathSuggestion(p.to_string());
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

    (tokens.iter().map(|s| s.as_str()).collect(), None)
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

fn main() {
    let cmd = Command::new(
        "/settings/user print",
        "Print user settings lorem ipsum dolorem set ami",
        |_| Err("".to_string()),
    );

    println!("{:?}", cmd.match_path("/settings/user print"));
    println!("{:?}", cmd.match_path("/settings/user print all"));
    println!("{:?}", cmd.match_path("/settings/user print all params"));
    println!("{:?}", cmd.match_path("/settings/user print all a=1 b=2"));
    println!("{:?}", cmd.match_path("/settings/user pri"));
    println!("{:?}", cmd.match_path("/settings/user"));
    println!("{:?}", cmd.match_path("/settings/us"));

    let cmd = Command::new("/note create", "Create a new note", |_| Err("".to_string()));

    println!("{:?}", cmd.match_path("/note create new"));
    println!("{:?}", cmd.match_path("/note create"));
    println!("{:?}", cmd.match_path("/note cr"));
    println!("{:?}", cmd.match_path("/note"));
    println!("{:?}", cmd.match_path("/no"));

    // println!("{:?}", &r);
}
