mod journal;

use std::path::PathBuf;
use std::{fs, process};

pub use journal::{Entry, Journal};

// TODO: add config
const DEFAULT_JOURNAL_DIR: &str = ".jot";
const DEFAULT_JOURNAL_SAVE_FILE: &str = "journal.json";

pub fn load_journal() -> std::io::Result<Journal> {
    let path = get_journal_path().unwrap_or(PathBuf::from(DEFAULT_JOURNAL_SAVE_FILE));
    load_from_path(path)
}

pub fn load_from_path(path: PathBuf) -> std::io::Result<Journal> {
    match fs::read_to_string(&path) {
        Ok(content) => {
            let entries: Vec<Entry> = serde_json::from_str(&content)?;
            Ok(Journal::from_entries(path, entries))
        }
        Err(_) => Ok(Journal::new(path)), // Return an empty journal if the file doesn't exist
    }
}

pub fn save_journal(journal: &Journal) -> std::io::Result<()> {
    let serialized_entries = serde_json::to_string(journal.entries())?;
    match fs::write(journal.path(), serialized_entries) {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("Journal could not be found, create one with `jot init`");
            } else {
                eprintln!("Error saving journal: {}", e);
            }
            process::exit(1);
        }
    }
}

/// Get the directory where the journal is stored
pub fn get_journal_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path;
    if cfg!(debug_assertions) {
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    } else {
        path = dirs::home_dir().ok_or("Could not find home directory")?;
    }

    path.push(DEFAULT_JOURNAL_DIR);
    Ok(path)
}

/// Get the path to the journal file
pub fn get_journal_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = get_journal_dir()?;
    path.push(DEFAULT_JOURNAL_SAVE_FILE);
    Ok(path)
}

pub fn init_journal() -> Result<(), Box<dyn std::error::Error>> {
    let journal_path = get_journal_path()?;
    std::fs::create_dir_all(get_journal_dir()?)?;

    if !journal_path.exists() {
        let journal = Journal::new(journal_path);
        save_journal(&journal)?;
        println!("Journal initialized");
    } else {
        println!("Journal already exists");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_journal_new() {
        let path = PathBuf::from("test.json");
        let journal = Journal::new(path.clone());
        assert_eq!(*journal.path(), path);
        assert!(journal.entries().is_empty());
    }

    #[test]
    fn test_entry_new() {
        let entry = Entry::new(0, "test".to_string(), vec![]);
        assert_eq!(entry.id, 0);
        assert_eq!(entry.body, "test");
    }

    #[test]
    fn test_get_journal_dir() -> Result<(), Box<dyn std::error::Error>> {
        let dir = get_journal_dir()?;
        assert!(dir.ends_with(DEFAULT_JOURNAL_DIR));
        Ok(())
    }

    #[test]
    fn test_get_journal_path() -> Result<(), Box<dyn std::error::Error>> {
        let path = get_journal_path()?;
        assert!(path.ends_with(DEFAULT_JOURNAL_SAVE_FILE));
        Ok(())
    }

    #[test]
    fn test_initialize() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().join(DEFAULT_JOURNAL_DIR);
        std::fs::create_dir_all(&path)?;

        init_journal()?;

        let journal_path = get_journal_path()?;
        assert!(journal_path.exists());
        Ok(())
    }

    #[test]
    fn test_load_nonexistent() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().join("nonexistent.json");

        let journal = load_from_path(path)?;
        assert!(journal.entries().is_empty());
        Ok(())
    }

    #[test]
    fn test_add_entry() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().join("test.json");

        let mut journal = Journal::new(path.clone());
        let entry = Entry::new(0, "test".to_string(), vec![]);
        journal.add_entry(entry);
        save_journal(&journal)?;

        let journal = load_from_path(path)?;
        assert_eq!(journal.entries().len(), 1);
        Ok(())
    }

    #[test]
    fn test_remove_entry() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().join("test.json");

        let mut journal = Journal::new(path.clone());
        let entry = Entry::new(0, "test".to_string(), vec![]);
        journal.add_entry(entry);
        save_journal(&journal)?;

        let entry = journal.remove_entry(0);
        assert!(entry.is_some());
        save_journal(&journal)?;

        let journal = load_from_path(path)?;
        assert!(journal.entries().is_empty());
        Ok(())
    }

    #[test]
    fn test_get_entry() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().join("test.json");

        let mut journal = Journal::new(path.clone());
        let entry = Entry::new(0, "test".to_string(), vec![]);
        journal.add_entry(entry);

        let entry = journal.get_entry(0);
        assert!(entry.is_some() && entry.unwrap().body == "test");
        Ok(())
    }
}
