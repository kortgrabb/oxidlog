use std::io::{self, Write};

use colored::Colorize;

use crate::storage::{config::JournalConfig, Entry, Tag};

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub enum TagMatch {
    Any,  // OR operation
    All,  // AND operation
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
    match chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            eprintln!("Invalid date format");
            std::process::exit(1);
        }
    }
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
        formatted.push_str(&format!(" {}", entry.tags.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(" ").bright_yellow()));
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
