use crate::{cli::ViewArgs, error::JotResult, storage::Journal, utils};

pub fn execute(
    journal: &Journal,
    args: ViewArgs, 
) -> JotResult<()> {
    if let Some(id) = args.id {
        if let Some(entry) = journal.entries().iter().find(|e| e.id == id) {
            println!("ID: {}", entry.id);
            println!("Date: {}", entry.date);
            println!("Body: {}", entry.body);
        } else {
            println!("Entry with id {id} not found");
        }
    } else {
        let entries = journal
            .get_entries()
            .iter()
            // Filter entries by date and tags
            .filter(|e| {
                if let Some(from) = &args.from {
                    let parsed_date = utils::parse_date(from);
                    if e.date < parsed_date {
                        return false;
                    }
                }
                if let Some(to) = &args.to {
                    let parsed_date = utils::parse_date(to);
                    if e.date > parsed_date {
                        return false;
                    }
                }
                utils::do_tags_match(&args.tags, &e.tags)
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
