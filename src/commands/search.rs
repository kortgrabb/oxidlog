use crate::{
    cli::SearchArgs,
    error::JotResult,
    storage::{config::Config, Journal},
    utils,
};

// TODO: Add date-filter, case-sensitive search, regex search

pub fn execute(journal: &Journal, args: SearchArgs, config: &Config) -> JotResult<()> {
    let term = args.query.to_lowercase();
    let entries = journal.get_entries();

    if entries.is_empty() {
        println!("No entries found.");
    } else {
        let found: Vec<String> = entries
            .iter()
            .filter(|e| {
                let content_matches = e.body.to_lowercase().contains(&term);

                if let Some(from) = &args.from {
                    let date = utils::parse_date(from);
                    if date < e.date {
                        return false;
                    }
                }

                if let Some(to) = &args.to {
                    let date = utils::parse_date(to);

                    if date > e.date {
                        return false;
                    }
                }

                utils::do_tags_match(&args.tags, &e.tags) && content_matches
            })
            .map(|e| utils::format_entry(e, config.journal_cfg.clone()))
            .collect();

        println!("{} entries found", found.len());
        found.iter().for_each(|e| println!("{}", e));
    }

    Ok(())
}
