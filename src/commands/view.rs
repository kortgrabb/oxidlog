use crate::{
    cli::ViewArgs,
    error::JotResult,
    storage::{config::Config, Entry, Journal},
    utils,
};

fn print_formatted_entries(entries: &[&Entry], config: &Config) {
    if entries.is_empty() {
        println!("No entries found.");
    } else {
        println!("{} entries found", entries.len());
        entries
            .iter()
            .for_each(|e| println!("{}", utils::format_entry(e, config.journal_cfg.clone())));
    }
}

pub fn execute(journal: &Journal, args: ViewArgs, config: &Config) -> JotResult<()> {
    if let Some(id) = args.id {
        if let Some(entry) = journal.entries().iter().find(|e| e.id == id) {
            println!("ID: {}", entry.id);
            println!("Date: {}", entry.date);
            println!("Body: {}", entry.body);
            println!("Tags: {}", entry.tags.join(", "));
        } else {
            println!("Entry with id {id} not found");
        }
    } else if args.recent {
        let entries = journal.get_entries();
        if let Some(last) = entries.iter().last() {
            print_formatted_entries(&[last], config);
        }
    } else {
        let entries = journal
            .get_entries()
            .iter()
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

        print_formatted_entries(&entries, config);
    }

    Ok(())
}
