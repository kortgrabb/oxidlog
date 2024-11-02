use chrono::{DateTime, NaiveDate, Utc};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

use crate::storage::load_config;

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub id: usize,
    pub timestamp: DateTime<Utc>,
    pub date: NaiveDate,
    pub body: String,
    pub tags: Vec<String>,
}

impl fmt::Display for Entry {
    // TODO: add extensive formatting configuration
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let config = load_config().unwrap_or_default();

        let mut date = self.timestamp.format("%Y-%m-%d").to_string();
        if config.journal_cfg.show_time {
            date = format!(
                "{} {}",
                self.timestamp.format("%Y-%m-%d").to_string().bright_black(),
                self.timestamp
                    .format("%H:%M")
                    .to_string()
                    .underline()
                    .bright_black()
            );
        }

        // Make the ID more prominent with bold
        let id_str = format!("[{}]", self.id).cyan().bold();

        // Style tags with a more subtle look
        let tags_str = self
            .tags
            .iter()
            .map(|t| t.on_blue().black().to_string())
            .collect::<Vec<_>>()
            .join(" ");

        let header = format!("{} {} {}", id_str, date, tags_str);

        // Highlight tag mentions in the body with a more subtle style
        let body = self.tags.iter().fold(self.body.clone(), |acc, tag| {
            acc.replace(tag, &tag.to_string().bright_blue().to_string())
        });

        // Add a separator line between entries
        write!(f, "{}\n{}\n{}", header, body, "─".repeat(40).bright_black())
    }
}

impl Entry {
    pub fn new(id: usize, body: String, tags: Vec<String>) -> Self {
        Self {
            id,
            timestamp: Utc::now(),
            date: Utc::now().naive_utc().date(),
            body,
            tags,
        }
    }
}

// #[derive(Default)]
// pub struct EntryBuilder {
//     id: Option<usize>,
//     content: Option<String>,
//     tags: Option<Vec<String>>,
// }

// impl EntryBuilder {
//     pub fn id(mut self, id: usize) -> Self {
//         self.id = Some(id);
//         self
//     }

//     pub fn content(mut self, content: String) -> Self {
//         self.content = Some(content);
//         self
//     }

//     pub fn tags(mut self, tags: Vec<String>) -> Self {
//         self.tags = Some(tags);
//         self
//     }

//     pub fn build(self) -> Entry {
//         Entry {
//             id: self.id.expect("id is required"),
//             timestamp: Utc::now(),
//             date: Utc::now().naive_utc().date(),
//             body: self.content.expect("content is required"),
//             tags: self.tags.unwrap_or_default(),
//         }
//     }
// }

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

    pub fn add_entry(&mut self, entry: Entry) {
        let id = self.entries.len();
        let entry = Entry { id, ..entry };

        self.entries.push(entry);
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
                body: old_body,
                ..entry.clone()
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

    pub fn next_id(&self) -> usize {
        self.entries.len()
    }
}
