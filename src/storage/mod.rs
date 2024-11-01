mod journal;

use std::fs;
use std::path::PathBuf;

pub use journal::{Entry, Journal};

const DEFAULT_JOURNAL_DIR: &str = ".journal";
const DEFAULT_JOURNAL_SAVE_FILE: &str = "journal.json";

pub fn load_journal() -> std::io::Result<Journal> {
    let path = get_journal_path().unwrap();
    load_from_path(path)
}

pub fn load_from_path(path: PathBuf) -> std::io::Result<Journal> {
    match fs::read_to_string(&path) {
        Ok(content) => {
            let entries: Vec<Entry> = serde_json::from_str(&content)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            Ok(Journal::from_entries(path, entries))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Journal::new(path)),
        Err(e) => Err(e),
    }
}

pub fn save_journal(journal: &Journal) -> std::io::Result<()> {
    let serialized_entries = serde_json::to_string(journal.entries())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    fs::write(journal.path(), serialized_entries)
}

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
    fn test_entry_builder() {
        let entry = Entry::builder().id(0).content("test".to_string()).build();
        assert_eq!(entry.id, 0);
        assert_eq!(entry.content, "test");
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
        journal.add_entry("test".to_string());

        let journal = load_from_path(path)?;
        assert_eq!(journal.entries().len(), 1);
        Ok(())
    }

    #[test]
    fn test_remove_entry() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().join("test.json");

        let mut journal = Journal::new(path.clone());
        journal.add_entry("test".to_string());

        let entry = journal.remove_entry(0);
        assert!(entry.is_some());

        let journal = load_from_path(path)?;
        assert!(journal.entries().is_empty());
        Ok(())
    }

    #[test]
    fn test_get_entry() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().join("test.json");

        let mut journal = Journal::new(path.clone());
        journal.add_entry("test".to_string());

        let entry = journal.get_entry(0);
        assert!(entry.is_some() && entry.unwrap().content == "test");
        Ok(())
    }
}
