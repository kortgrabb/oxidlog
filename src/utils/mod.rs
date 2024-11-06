use std::io::{self, Write};

use colored::Colorize;

use crate::storage::Entry;

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn do_tags_match(tags: &[String], entry_tags: &[String]) -> bool {
    if tags.is_empty() {
        return true;
    }
    entry_tags.iter().any(|t| tags.contains(&t.to_string()))
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

pub fn format_entry(entry: &Entry, show_time: bool) -> String {
    let mut formatted = String::new();
    formatted.push_str(
        &format!(
            "[{:>3}] {}",
            entry.id,
            entry.date.format("%Y-%m-%d").to_string().bright_blue()
        )
        .to_string(),
    );

    if show_time {
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
        formatted.push_str(&format!(" {}", entry.tags.join(", ").italic().dimmed()));
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
