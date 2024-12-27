use colored::Colorize;

use crate::error::{JotError, JotResult};
use crate::storage::config::Config;
use crate::storage::{self, Entry, Journal};

pub fn execute(journal: &mut Journal, content: String, _config: &Config) -> JotResult<()> {
    let tags = content
        .split_whitespace()
        .filter(|w| w.starts_with('#'))
        .map(|w| w[1..].to_string())
        .collect();
    let body = String::from(content.trim());
    if body.is_empty() {
        return Err(JotError::AddError("Entry cannot be empty".to_string()));
    }

    let entry = Entry::new(journal.next_id(), body, tags);
    journal.add_entry(entry);
    storage::save_journal(journal)?;

    println!(
        "Entry {} added!",
        format!("#{}", journal.next_id()).bold().green()
    );

    Ok(())
}
