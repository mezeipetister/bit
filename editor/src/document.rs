use std::{
    io::BufWriter,
    ops::{Deref, DerefMut},
};

use crate::{editor::Position, row::Row};

pub struct Document<F>
where
    F: FnMut(String) -> Result<(), String>,
{
    pub title: String,
    pub(crate) rows: Vec<Row>,
    dirty: bool,
    on_save: F,
}

impl<F> Document<F>
where
    F: FnMut(String) -> Result<(), String>,
{
    pub fn new(title: String, content: String, on_save: F) -> Self {
        let mut rows = Vec::new();

        for value in content.lines() {
            rows.push(Row::from(value));
        }

        Self {
            title,
            rows,
            dirty: false,
            on_save,
        }
    }
    pub fn save(&mut self) -> Result<(), String> {
        let content = self.as_string();
        (self.on_save)(content)?;
        self.dirty = false;
        Ok(())
    }
    pub fn as_string(&self) -> String {
        let mut res = String::new();
        for row in &self.rows {
            res.push_str(row.as_str());
            res.push_str("\r\n");
        }
        res
    }
    pub(crate) fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    fn insert_newline(&mut self, at: &Position) {
        if at.y > self.rows.len() {
            return;
        }
        if at.y == self.rows.len() {
            self.rows.push(Row::default());
            return;
        }
        #[allow(clippy::indexing_slicing)]
        let current_row = &mut self.rows[at.y];
        let new_row = current_row.split(at.x);
        #[allow(clippy::integer_arithmetic)]
        self.rows.insert(at.y + 1, new_row);
    }
    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.rows.len() {
            return;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_newline(at);
        } else if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            #[allow(clippy::indexing_slicing)]
            let row = &mut self.rows[at.y];
            row.insert(at.x, c);
        }
    }
    #[allow(clippy::integer_arithmetic, clippy::indexing_slicing)]
    pub fn delete(&mut self, at: &Position) {
        let len = self.rows.len();
        if at.y >= len {
            return;
        }
        self.dirty = true;
        if at.x == self.rows[at.y].len() && at.y + 1 < len {
            let next_row = self.rows.remove(at.y + 1);
            let row = &mut self.rows[at.y];
            row.append(&next_row);
        } else {
            let row = &mut self.rows[at.y];
            row.delete(at.x);
        }
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}
