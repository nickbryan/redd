use anyhow::{Context, Result};
use std::fs;

use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

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

    pub fn len(&self) -> usize {
        self.len
    }

    fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count()
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
