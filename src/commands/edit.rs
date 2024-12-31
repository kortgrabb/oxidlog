use crate::{
    error::{JotError, JotResult},
    storage::{self, Entry, Journal, Tag},
    utils,
};
use colored::Colorize;

#[derive(clap::Args, Clone)]
pub struct EditArgs {
    pub id: usize,
}

pub fn execute(journal: &mut Journal, args: EditArgs) -> JotResult<()> {
    let id = args.id;
    match journal.get_entry(id) {
        Some(entry) => {
            println!("Editing entry: {}", entry.body);
            let new_body = utils::get_input(&format!("Enter new content [{}]: ", entry.body));
            let tags_str = entry.tags.iter().map(|t| t.name.clone()).collect::<Vec<_>>().join(" ");
            let new_tags: Vec<Tag> =
                utils::get_input(&format!("Enter new tags [{}]: ", tags_str))
                    .split_whitespace()
                    .map(|s| Tag::new(s.to_string()))
                    .collect();

            let new_entry = Entry {
                id: entry.id,
                body: new_body,
                tags: new_tags,
                date: entry.date,
                timestamp: entry.timestamp,
            };

            journal.update_entry(new_entry);
            storage::save_journal(journal)?;

            println!("{}", "Entry updated!".green());

            Ok(())
        }
        None => Err(JotError::EditError(format!(
            "Entry with ID {} not found",
            id
        ))),
    }
}
