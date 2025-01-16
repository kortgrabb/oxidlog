use colored::Colorize;

use crate::error::{JotError, JotResult};
use crate::storage::config::Config;
use crate::storage::{self, Entry, Journal, Tag};

#[derive(clap::Args, Clone)]
pub struct AddArgs {
    pub content: String,
}

pub fn execute(journal: &mut Journal, args: AddArgs, config: &Config) -> JotResult<()> {
    let content = args.content.trim();
    if content.is_empty() {
        return Err(JotError::AddError("Entry cannot be empty".to_string()));
    }

    let tags: Vec<Tag> = content
        .split_whitespace()
        .filter(|w| w.starts_with('#'))
        .map(|w| Tag::new(w[1..].to_string()))
        .collect();

    let body = if config.journal_cfg.body_tags {
        content
            .split_whitespace()
            .filter(|w| !w.starts_with('#'))
            .collect::<Vec<&str>>()
            .join(" ")
    } else {
        content.to_string()
    };

    let entry = Entry::new(journal.next_id(), body, tags);
    journal.add_entry(entry);
    storage::save_journal(journal)?;

    println!(
        "Entry {} added!",
        format!("#{}", journal.next_id()).bold().green()
    );

    Ok(())
}
