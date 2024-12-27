use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub id: usize,
    pub timestamp: DateTime<Utc>,
    pub date: NaiveDate,
    pub body: String,
    pub tags: Vec<String>,
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

    pub fn update_entry(&mut self, entry: Entry) {
        if let Some(index) = self.entries.iter().position(|e| e.id == entry.id) {
            self.entries[index] = entry;
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_entry_creation() {
        let entry = Entry::new(1, "Test entry".to_string(), vec!["test".to_string()]);
        assert_eq!(entry.id, 1);
        assert_eq!(entry.body, "Test entry");
        assert_eq!(entry.tags, vec!["test"]);
        assert!(entry.timestamp <= Utc::now());
    }

    #[test]
    fn test_journal_operations() {
        let path = PathBuf::from("test_journal.json");
        let mut journal = Journal::new(path.clone());

        // Test adding entries
        journal.add_entry(Entry::new(0, "First entry".to_string(), vec![]));
        journal.add_entry(Entry::new(
            0,
            "Second entry".to_string(),
            vec!["tag1".to_string()],
        ));

        assert_eq!(journal.entries().len(), 2);
        assert_eq!(journal.get_entry(0).unwrap().body, "First entry");
        assert_eq!(journal.get_entry(1).unwrap().body, "Second entry");

        // Test removing entries
        let removed = journal.remove_entry(0);
        assert!(removed.is_some());
        assert_eq!(journal.entries().len(), 1);

        // Test updating entries
        let mut entry = journal.get_entry(1).unwrap().clone();
        entry.body = "Updated entry".to_string();
        journal.update_entry(entry);
        assert_eq!(journal.get_entry(1).unwrap().body, "Updated entry");
    }

    #[test]
    fn test_journal_next_id() {
        let path = PathBuf::from("test_journal.json");
        let mut journal = Journal::new(path);

        assert_eq!(journal.next_id(), 0);
        journal.add_entry(Entry::new(0, "Entry".to_string(), vec![]));
        assert_eq!(journal.next_id(), 1);
    }
}
