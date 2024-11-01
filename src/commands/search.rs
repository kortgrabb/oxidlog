use crate::{
    error::JotResult,
    storage::{Entry, Journal},
};

// TODO: search by tags
pub fn execute(journal: &Journal, query: &str, tags: Vec<String>) -> JotResult<()> {
    let term = query.to_lowercase();
    let entries = journal.get_entries();

    if entries.is_empty() {
        println!("No entries found.");
    } else {
        let found: Vec<String> = entries
            .iter()
            .filter(|e| {
                let content_matches = e.body.to_lowercase().contains(&term);
                let tag_matches = tags.is_empty()
                    || e.tags
                        .iter()
                        .map(|t| t.trim_start_matches('#'))
                        .any(|t| tags.contains(&t.to_string()));

                content_matches && tag_matches
            })
            .map(|e| format!("{}", e))
            .collect();

        println!("{} entries found", found.len());
        found.iter().for_each(|e| println!("{}", e));
    }

    Ok(())
}
