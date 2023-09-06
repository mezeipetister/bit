use std::iter::Peekable;

#[derive(Debug, Default)]
struct RawExpression {
    tokens: Vec<RawToken>,
}

impl RawExpression {
    fn start_position(&self) -> (usize, usize) {
        self.tokens
            .first()
            .map(|i| i.start_position())
            .unwrap_or_default()
    }
    fn end_position(&self) -> (usize, usize) {
        self.tokens
            .last()
            .map(|i| i.end_position())
            .unwrap_or_default()
    }
    fn position_range(&self) -> ((usize, usize), (usize, usize)) {
        (self.start_position(), self.end_position())
    }
    fn new_empty() -> Self {
        Self { tokens: vec![] }
    }
    fn char_to_last_token(&mut self) {
        self
    }
}

#[derive(Debug)]
struct RawToken {
    token: Token,
    cursor: (usize, usize),
}

impl RawToken {
    fn start_position(&self) -> (usize, usize) {
        (self.cursor.0, self.cursor.1)
    }
    fn end_position(&self) -> (usize, usize) {
        (self.cursor.0, self.cursor.1 + self.token.len())
    }
}

struct Parser {
    cursor: (usize, usize),
    expressions: Vec<RawExpression>,
    paren_mode: bool,
}

impl Parser {
    fn new() -> Self {
        Self {
            cursor: (1, 1),
            expressions: vec![RawExpression::default()],
            paren_mode: false,
        }
    }
    fn char_to_token(&mut self, char: &char) {}
    fn char_to_new_token(&mut self, char: &char) {}
    fn char_to_new_expr(&mut self, char: &char) {}
    fn process(mut self, mut i: Peekable<impl Iterator<Item = char>>) -> Self {
        if let Some(current) = i.next() {
            match current {
                '\n' => {
                    self.cursor.0 += 1;
                    self.cursor.1 = 0;
                }
                '"' => {
                    self.paren_mode = !self.paren_mode;
                }
                ' ' => match self.paren_mode {
                    true => match &mut self.expressions.last_mut().unwrap().tokens.last_mut() {
                        Some(last_token) => unimplemented!(),
                        None => panic!("paren mode but no buffer found.."),
                    },
                    false => {
                        self.token_buffer = None;
                    }
                },
                x => match &mut self.token_buffer {
                    Some(b) => b.0.push(x),
                    None => {
                        self.token_buffer = Some((String::from(x), (self.cursor.0, self.cursor.1)))
                    }
                },
            }
            self.cursor.1 += 1;
            let res = self.process(i);
            res
        } else {
            self
        }
    }
    fn parse(self, i: &str) -> Self {
        let mut i = i.chars().peekable();
        self.process(i)
    }
}

#[derive(Debug)]
enum Token {
    Key(String),
    Value(String),
}

impl Token {
    fn len(&self) -> usize {
        match self {
            Token::Key(s) => s.len(),
            Token::Value(s) => s.len(),
        }
    }
}

fn main() {
    let s = r#"Hello bello

a
b
lorem ipsum    dolorem."#;
    // process(s);
    Parser::new().parse(s);
}
