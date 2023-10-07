use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

use crate::cli::Context;

pub(crate) struct Row {
    prefix: String,
    user_input: String,
    len: usize,
}

impl Row {
    pub fn new(ctx: &Context, slice: &str) -> Self {
        Self {
            prefix: create_prefix(ctx),
            user_input: String::from(slice),
            len: slice.graphemes(true).count(),
        }
    }
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.user_input.len());
        let start = cmp::min(start, end);
        let mut result = String::new();
        #[allow(clippy::integer_arithmetic)]
        for (index, grapheme) in self.user_input[..]
            .graphemes(true)
            .enumerate()
            .skip(start)
            .take(end - start)
        {
            if let Some(c) = grapheme.chars().next() {
                if c == '\t' {
                    result.push_str(" ");
                } else {
                    result.push(c);
                }
            }
        }
        result
    }
    pub fn as_str(&self) -> &str {
        &self.user_input
    }
    pub fn display(&self) -> String {
        format!("{}{}", self.prefix, &self.user_input)
    }
    pub fn as_string(self) -> String {
        self.user_input
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn prefix_len(&self) -> usize {
        self.prefix.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.user_input.push(c);
            self.len += 1;
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.user_input[..].graphemes(true).enumerate() {
            length += 1;
            if index == at {
                length += 1;
                result.push(c);
            }
            result.push_str(grapheme);
        }
        self.len = length;
        self.user_input = result;
    }
    pub fn append(&mut self, new: &Self) {
        self.user_input = format!("{}{}", self.user_input, new.user_input);
        self.len += new.len;
    }
    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.user_input[..].graphemes(true).enumerate() {
            if index != at {
                length += 1;
                result.push_str(grapheme);
            }
        }
        self.len = length;
        self.user_input = result;
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.user_input.as_bytes()
    }
}

fn create_prefix(ctx: &Context) -> String {
    format!("{}@{} {}: ", &ctx.user, &ctx.repo, &ctx.path)
}
