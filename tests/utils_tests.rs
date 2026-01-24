use tempfile::tempdir;
use std::io::{self, Write};
use crate::utils::{get_input, do_tags_match, parse_tags, parse_date, format_entry, fuzzy_match, view_by_id, print_single_entry};
use crate::storage::{Entry, Tag, Journal};
use crate::storage::config::JournalConfig;
use chrono::{NaiveDate, Utc};

#[test]
fn test_get_input() {
    let input = b"Test input\n";
    let mut stdin = io::stdin();
    stdin.write_all(input).unwrap();
    let result = get_input("Enter input: ");
    assert_eq!(result, "Test input");
}

#[test]
fn test_do_tags_match_any() {
    let query_tags = vec![Tag::new("test".to_string()), Tag::new("example".to_string())];
    let entry_tags = vec![Tag::new("example".to_string()), Tag::new("sample".to_string())];
    assert!(do_tags_match(&query_tags, &entry_tags, TagMatch::Any));
}

#[test]
fn test_do_tags_match_all() {
    let query_tags = vec![Tag::new("test".to_string()), Tag::new("example".to_string())];
    let entry_tags = vec![Tag::new("test".to_string()), Tag::new("example".to_string()), Tag::new("sample".to_string())];
    assert!(do_tags_match(&query_tags, &entry_tags, TagMatch::All));
}

#[test]
fn test_parse_tags() {
    let tags_str = "test example sample";
    let tags = parse_tags(tags_str);
    assert_eq!(tags.len(), 3);
    assert_eq!(tags[0].name, "test");
    assert_eq!(tags[1].name, "example");
    assert_eq!(tags[2].name, "sample");
}

#[test]
fn test_parse_date() {
    let date_str = "2023-09-15";
    let date = parse_date(date_str);
    assert_eq!(date, NaiveDate::from_ymd(2023, 9, 15));
}

#[test]
fn test_format_entry() {
    let entry = Entry::new(1, "Test entry".to_string(), vec![Tag::new("test".to_string())]);
    let config = JournalConfig { body_tags: true, show_time: true, export_dir: "exports".to_string() };
    let formatted = format_entry(&entry, config);
    assert!(formatted.contains("Test entry"));
    assert!(formatted.contains("#test"));
}

#[test]
fn test_fuzzy_match() {
    assert!(fuzzy_match("fuzzy matching", "fuz mat"));
    assert!(!fuzzy_match("fuzzy matching", "fuzzy not matching"));
}

#[test]
fn test_view_by_id() {
    let temp_dir = tempdir().unwrap();
    let journal_path = temp_dir.path().join("journal.json");
    let mut journal = Journal::new(journal_path.clone());
    journal.add_entry(Entry::new(1, "Test entry".to_string(), vec![Tag::new("test".to_string())]));

    let mut stdout = io::stdout();
    let result = view_by_id(&journal, 1);
    assert!(result.contains("Test entry"));
}

#[test]
fn test_print_single_entry() {
    let entry = Entry::new(1, "Test entry".to_string(), vec![Tag::new("test".to_string())]);
    let mut stdout = io::stdout();
    print_single_entry(&entry);
    let output = stdout.to_string();
    assert!(output.contains("Test entry"));
    assert!(output.contains("#test"));
}
