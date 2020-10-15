use crate::row::Row;
use anyhow::{Context, Result};
use std::fs;

#[derive(Default)]
pub struct Document {
    name: String,
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
            name: String::from(filename),
            rows,
        })
    }

    pub fn name(&self) -> &String {
        &self.name
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
