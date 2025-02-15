use tempfile::tempdir;
use std::fs;
use std::path::PathBuf;
use crate::storage::{self, Journal, Entry, Tag};

#[test]
fn test_load_journal() {
    let temp_dir = tempdir().unwrap();
    let journal_path = temp_dir.path().join("journal.json");
    fs::write(&journal_path, "[]").unwrap();

    let journal = storage::load_from_path(journal_path.clone()).unwrap();
    assert!(journal.entries().is_empty());
    assert_eq!(journal.path(), &journal_path);
}

#[test]
fn test_save_journal() {
    let temp_dir = tempdir().unwrap();
    let journal_path = temp_dir.path().join("journal.json");

    let mut journal = Journal::new(journal_path.clone());
    journal.add_entry(Entry::new(0, "Test entry".to_string(), vec![Tag::new("test".to_string())]));

    storage::save_journal(&journal).unwrap();
    assert!(journal_path.exists());

    let loaded_journal = storage::load_from_path(journal_path).unwrap();
    assert_eq!(loaded_journal.entries().len(), 1);
    assert_eq!(loaded_journal.entries()[0].body, "Test entry");
    assert_eq!(loaded_journal.entries()[0].tags[0].name, "test");
}

#[test]
fn test_init_journal() {
    let temp_dir = tempdir().unwrap();
    let journal_path = temp_dir.path().join("journal.json");
    let config = storage::config::Config::default();

    storage::init_journal(&config).unwrap();
    assert!(journal_path.exists());

    let journal = storage::load_from_path(journal_path).unwrap();
    assert!(journal.entries().is_empty());
}
