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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let config = load_config().unwrap_or_default();

        // Format date and time
        let date_time = if config.journal_cfg.show_time {
            format!(
                "{} {}",
                self.timestamp.format("%Y-%m-%d").to_string().bright_black(),
                self.timestamp.format("%H:%M").to_string().bright_black()
            )
        } else {
            self.timestamp.format("%Y-%m-%d").to_string().bright_black().to_string()
        };

        // Format ID with a fixed width for better alignment
        let id_str = format!("[{:>3}]", self.id).cyan().bold();

        // Format tags with a more modern look
        let tags_str = if !self.tags.is_empty() {
            self.tags
                .iter()
                .map(|t| format!("#{}", t).bright_blue().to_string())
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            String::new()
        };

        // Create the header with proper spacing
        let header = if tags_str.is_empty() {
            format!("{} {}", id_str, date_time)
        } else {
            format!("{} {} {}", id_str, date_time, tags_str)
        };

        // Process the body: highlight tag mentions and add proper indentation
        let processed_body = self.tags.iter().fold(self.body.clone(), |acc, tag| {
            acc.replace(&format!("#{}", tag), &format!("#{}", tag).bright_blue())
        });

        // Add a subtle separator line
        let separator = "─".repeat(50).bright_black();

        // Combine all elements with proper spacing
        write!(
            f,
            "{}\n{}\n{}",
            header,
            processed_body.trim(),
            separator
        )
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
