pub struct Tokens {
    pub tokens: Vec<String>,
    pub position: usize,
}

impl Iterator for Tokens {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.tokens.len() {
            let res = self.tokens[self.position].clone();
            self.position += 1;
            Some(res)
        } else {
            None
        }
    }
}

impl Tokens {
    pub fn cmd(&self) -> Option<&str> {
        self.tokens.get(0).map(|s| s.as_str())
    }
    pub fn get(&self, i: usize) -> Option<&str> {
        self.tokens.get(i).map(|s| s.as_str())
    }
}

pub fn parse_input(input: &str) -> Tokens {
    let mut res = Vec::new();
    let mut word = String::new();
    let mut inside_quotes = false;

    for c in input.chars() {
        if c == '"' {
            inside_quotes = !inside_quotes;
            if !word.is_empty() {
                res.push(word.clone());
                word.clear();
            }
        } else if c == ' ' && !inside_quotes {
            if !word.is_empty() {
                res.push(word.clone());
                word.clear();
            }
        } else {
            word.push(c);
        }
    }

    if !word.is_empty() {
        res.push(word);
    }

    Tokens {
        tokens: res,
        position: 0,
    }
}
