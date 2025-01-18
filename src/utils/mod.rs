use std::io::{self, Write};

use colored::Colorize;

use crate::storage::{config::JournalConfig, Entry, Journal, Tag};

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub enum TagMatch {
    Any, // OR operation
    All, // AND operation
}

pub fn do_tags_match(query_tags: &[Tag], entry_tags: &[Tag], match_type: TagMatch) -> bool {
    if query_tags.is_empty() {
        return true;
    }

    match match_type {
        TagMatch::Any => query_tags.iter().any(|tag| entry_tags.contains(tag)),
        TagMatch::All => query_tags.iter().all(|tag| entry_tags.contains(tag)),
    }
}

// Helper function for parsing multiple tags from strings
pub fn parse_tags(tags: &str) -> Vec<Tag> {
    tags.split_whitespace()
        .map(|t| t.parse::<Tag>().unwrap())
        .collect()
}

pub fn parse_date(date: &str) -> chrono::NaiveDate {
    chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
}
pub fn format_entry(entry: &Entry, cfg: JournalConfig) -> String {
    let mut formatted = String::new();
    formatted.push_str(
        &format!(
            "[{:>3}] {}",
            entry.id,
            entry.date.format("%Y-%m-%d").to_string().bright_blue()
        )
        .to_string(),
    );

    if cfg.show_time {
        formatted.push_str(&format!(
            " {}",
            entry
                .timestamp
                .format("%H:%M")
                .to_string()
                .dimmed()
                .underline()
        ));
    }

    if !entry.tags.is_empty() {
        formatted.push_str(&format!(
            " {}",
            entry
                .tags
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(" ")
                .bright_yellow()
        ));
    }

    let body_colored = entry
        .body
        .split_whitespace()
        .map(|word| {
            if word.starts_with('#') {
                word.bright_green().to_string()
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<String>>();

    formatted.push_str(&format!("\n{}\n", body_colored.join(" ")));
    formatted.push_str(&"-".repeat(40));

    formatted
}

pub fn fuzzy_match(haystack: &str, needle: &str) -> bool {
    let needle_chars = needle.chars();
    let mut haystack_chars = haystack.chars();

    for needle_char in needle_chars {
        if let Some(haystack_char) = haystack_chars.next() {
            if needle_char == haystack_char {
                continue;
            } else {
                for next_haystack_char in haystack_chars.by_ref() {
                    if needle_char == next_haystack_char {
                        break;
                    }
                }
            }
        } else {
            return false;
        }
    }

    true
}

pub fn view_by_id(journal: &Journal, id: usize) {
    if let Some(entry) = journal.entries().iter().find(|e| e.id == id) {
        print_single_entry(entry);
    } else {
        println!("Entry with id {id} not found");
    }
}

pub fn print_single_entry(entry: &Entry) {
    println!("\n{}", "=".repeat(50));
    println!("Entry #{}", entry.id);
    println!("Date: {}", entry.date);
    println!("\n{}\n", entry.body);

    let tags = entry
        .tags
        .iter()
        .map(|t| format!("#{}", t.name))
        .collect::<Vec<_>>()
        .join(" ");

    if !tags.is_empty() {
        println!("Tags: {}", tags);
    }
    println!("{}", "=".repeat(50));
}
