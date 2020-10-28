use crate::ui::layout::Position;
use anyhow::{Context, Error, Result};
use std::fs;

use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl Row {
    pub fn to_string(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();

        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push_str(" ");
            } else {
                result.push_str(grapheme);
            }
        }

        result
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.update_len();
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            self.update_len();
            return;
        }

        let mut result: String = self.string[..].graphemes(true).take(at).collect();
        let remainder: String = self.string[..].graphemes(true).skip(at + 1).collect();
        result.push_str(&remainder);
        self.string = result;

        self.update_len();
    }

    pub fn insert(&mut self, at: usize, ch: char) {
        if at >= self.len() {
            self.string.push(ch);
            self.update_len();
            return;
        }

        let mut result: String = self.string[..].graphemes(true).take(at).collect();
        let remainder: String = self.string[..].graphemes(true).skip(at).collect();

        result.push(ch);
        result.push_str(&remainder);
        self.string = result;

        self.update_len();
    }

    pub fn split(&mut self, at: usize) -> Self {
        let beginning: String = self.string[..].graphemes(true).take(at).collect();
        let remainder: String = self.string[..].graphemes(true).skip(at).collect();
        self.string = beginning;
        self.update_len();
        Self::from(&remainder[..])
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            string: String::from(slice),
            len: 0,
        };

        row.update_len();
        row
    }
}

#[derive(Default)]
pub struct Document {
    file_name: Option<String>,
    rows: Vec<Row>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self> {
        let contents = fs::read_to_string(filename).context("unable to read from file")?;
        let mut rows = Vec::new();

        for row in contents.lines() {
            rows.push(Row::from(row));
        }

        Ok(Self {
            file_name: Some(String::from(filename)),
            rows,
        })
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        use {std::fs::File, std::io::Write};

        if let Some(file_name) = &self.file_name {
            let mut file = File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
        }

        Ok(())
    }

    pub fn delete(&mut self, at: &Position) {
        if at.y >= self.len() {
            return;
        }

        if at.x == self.rows.get_mut(at.y).unwrap().len() && at.y < self.len() - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next_row);
            return;
        }

        let row = self.rows.get_mut(at.y).unwrap();
        row.delete(at.x);
    }

    pub fn insert(&mut self, at: &Position, ch: char) -> Result<()> {
        use std::cmp::Ordering;

        match at.y.cmp(&self.len()) {
            Ordering::Equal => {
                let mut row = Row::default();
                row.insert(0, ch);
                self.rows.push(row);

                Ok(())
            }
            Ordering::Less => {
                let row = self.rows.get_mut(at.y).unwrap();
                row.insert(at.x, ch);

                Ok(())
            }
            Ordering::Greater => Err(Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "trying to insert character past current string length",
            ))),
        }
    }

    pub fn insert_newline(&mut self, at: &Position) {
        if at.y > self.len() {
            return;
        }

        if at.y == self.len() {
            self.rows.push(Row::default());
            return;
        }

        let new_row = self.rows.get_mut(at.y).unwrap().split(at.x);
        self.rows.insert(at.y + 1, new_row);
    }

    pub fn file_name(&self) -> Option<&String> {
        self.file_name.as_ref()
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
