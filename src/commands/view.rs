use crate::{
    error::JotResult,
    storage::{config::Config, Journal},
    utils,
};

pub fn execute(
    journal: &Journal,
    id: Option<usize>,
    from: Option<String>,
    to: Option<String>,
    tags: Vec<String>,
) -> JotResult<()> {
    if let Some(id) = id {
        if let Some(entry) = journal.entries().iter().find(|e| e.id == id) {
            println!("{}", entry);
        } else {
            println!("Entry #{} not found", id);
        }
    } else {
        let entries = journal.get_entries();
        let entries = entries
            .iter()
            // Filter entries by date and tags
            .filter(|e| {
                if let Some(from) = &from {
                    let parsed_date = utils::parse_date(from);
                    if e.date < parsed_date {
                        return false;
                    }
                }
                if let Some(to) = &to {
                    let parsed_date = utils::parse_date(to);
                    if e.date > parsed_date {
                        return false;
                    }
                }
                utils::do_tags_match(&tags, &e.tags)
            })
            .collect::<Vec<_>>();

        if entries.is_empty() {
            println!("No entries found.");
        } else {
            println!("{} entries found", entries.len());
            entries.iter().for_each(|e| println!("{}", e));
        }
    }

    Ok(())
}
