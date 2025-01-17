use crate::{
    error::JotResult,
    storage::{config::Config, Journal, Tag},
    utils::{self, TagMatch},
};

// TODO: Add date-filter, case-sensitive search, regex search

#[derive(clap::Args, Clone)]
pub struct SearchArgs {
    pub query: String,
    #[clap(long, value_delimiter = ' ')]
    pub tags: Vec<String>,
    #[clap(long)]
    pub from: Option<String>,
    #[clap(long)]
    pub to: Option<String>,
    #[clap(short, long)]
    pub fuzzy: bool,
    #[clap(short, long)]
    pub all: bool,
}

pub fn execute(journal: &Journal, args: SearchArgs, config: &Config) -> JotResult<()> {
    let term = args.query.to_lowercase();
    let entries = journal.get_entries();
    let do_fuzzy = args.fuzzy;

    if entries.is_empty() {
        println!("No entries found.");
    } else {
        let found: Vec<String> = entries
            .iter()
            .filter(|e| {
                let content = e.body.to_lowercase();

                let content_matches = if do_fuzzy {
                    utils::fuzzy_match(&content, &term)
                } else {
                    content.contains(&term)
                };

                if let Some(from) = &args.from {
                    let date = utils::parse_date(from);
                    if date > e.date {
                        return false;
                    }
                }

                if let Some(to) = &args.to {
                    let date = utils::parse_date(to);

                    if date < e.date {
                        return false;
                    }
                };

                let match_type = if args.all {
                    TagMatch::All
                } else {
                    TagMatch::Any
                };

                utils::do_tags_match(
                    &args
                        .tags
                        .iter()
                        .map(|t| Tag::new(t.to_string()))
                        .collect::<Vec<_>>(),
                    &e.tags,
                    match_type,
                ) && content_matches
            })
            .map(|e| utils::format_entry(e, config.journal_cfg.clone()))
            .collect();

        println!("{} entries found", found.len());
        found.iter().for_each(|e| {
            if term.is_empty() {
                println!("{}", e);
            } else {
                let highlighted = e.replace(&term, &format!("\x1b[42m{}\x1b[0m", &term));
                println!("{}", highlighted);
            }
        });
    }

    Ok(())
}
