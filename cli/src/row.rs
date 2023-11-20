use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub(crate) struct Row {
    pre: String,
    input: String,
    length: usize,
    previous_length: usize,
    pub position: usize,
}

impl Row {
    pub fn new(slice: &str) -> Self {
        Self {
            pre: "bit > ".to_string(),
            input: String::from(slice),
            length: slice.graphemes(true).count(),
            previous_length: 0,
            position: slice.graphemes(true).count(),
        }
    }
    // pub fn render(&self, start: usize, end: usize) -> String {
    //     let end = cmp::min(end, self.input.len());
    //     let start = cmp::min(start, end);
    //     let mut result = String::new();
    //     #[allow(clippy::integer_arithmetic)]
    //     for (index, grapheme) in self.input[..]
    //         .graphemes(true)
    //         .enumerate()
    //         .skip(start)
    //         .take(end - start)
    //     {
    //         if let Some(c) = grapheme.chars().next() {
    //             if c == '\t' {
    //                 result.push_str(" ");
    //             } else {
    //                 result.push(c);
    //             }
    //         }
    //     }
    //     result
    // }
    pub fn display(&self) -> String {
        let mut res = format!("{}{}", &self.pre, &self.input);
        let len = res.graphemes(true).count();
        let (width, _) = termion::terminal_size().unwrap();
        for _ in 0..width - len as u16 {
            res.push_str(" ");
        }
        res
    }
    pub fn go_left(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }
    pub fn go_right(&mut self) {
        if self.position < self.length {
            self.position += 1;
        }
    }
    pub fn insert(&mut self, c: char) {
        // Set previous length
        self.previous_length = self.input.graphemes(true).count();

        let mut result: String = String::new();

        for (index, grapheme) in self.input[..].graphemes(true).enumerate() {
            if index == self.position {
                result.push(c);
            }
            result.push_str(grapheme);
        }

        if self.length == self.position {
            result.push(c);
        }

        // Set length
        self.length = result.graphemes(true).count();

        // Set new input
        self.input = result;

        // Move cursor right
        self.go_right();
    }
    pub fn delete(&mut self) {
        // Set previous length
        self.previous_length = self.input.graphemes(true).count();

        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.input[..].graphemes(true).enumerate() {
            if index != self.position {
                length += 1;
                result.push_str(grapheme);
            }
        }
        self.length = length;
        self.input = result;
    }
    pub fn backspace(&mut self) {
        // Move cursor left
        self.go_left();
        // Perform a delete action
        self.delete();
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.input.as_bytes()
    }
    pub fn as_str(&self) -> &str {
        &self.input
    }
    pub fn position(&self) -> usize {
        self.position + self.pre.graphemes(true).count()
    }
}
