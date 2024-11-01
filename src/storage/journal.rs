use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub id: usize,
    pub timestamp: DateTime<Utc>,
    pub body: String,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.id,
            self.timestamp.format("%Y-%m-%d %H:%M:%S"),
            self.body
        )
    }
}

impl Entry {
    pub fn builder() -> EntryBuilder {
        EntryBuilder::default()
    }
}

#[derive(Default)]
pub struct EntryBuilder {
    id: Option<usize>,
    content: Option<String>,
}

impl EntryBuilder {
    pub fn id(mut self, id: usize) -> Self {
        self.id = Some(id);
        self
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn build(self) -> Entry {
        Entry {
            id: self.id.expect("id is required"),
            timestamp: Utc::now(),
            body: self.content.expect("content is required"),
        }
    }
}

pub struct Journal {
    path: PathBuf,
    entries: Vec<Entry>,
}

impl Journal {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            entries: Vec::new(),
        }
    }

    pub fn from_entries(path: PathBuf, entries: Vec<Entry>) -> Self {
        Self { path, entries }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    pub fn add_entry(&mut self, content: String) -> Entry {
        let next_id = self.entries.len();
        let entry = Entry::builder().id(next_id).content(content).build();
        self.entries.push(entry.clone());
        entry
    }

    pub fn remove_entry(&mut self, id: usize) -> Option<Entry> {
        if let Some(index) = self.entries.iter().position(|e| e.id == id) {
            Some(self.entries.remove(index))
        } else {
            None
        }
    }

    pub fn edit_entry(&mut self, id: usize, new_body: &str) -> Option<Entry> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            let old_body = entry.body.clone();
            entry.body = new_body.to_string();
            Some(Entry {
                id: entry.id,
                timestamp: entry.timestamp,
                body: old_body,
            })
        } else {
            None
        }
    }

    pub fn get_entry(&self, id: usize) -> Option<&Entry> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn get_entries(&self) -> &[Entry] {
        &self.entries
    }
}
