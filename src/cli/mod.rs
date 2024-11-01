use crate::error::JotResult;
use crate::{commands, storage};
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add {
        entry: String,
    },
    Remove {
        id: usize,
        from: Option<String>,
        #[clap(short, long)]
        to: Option<String>,
    },
    View {
        #[clap(short, long)]
        id: Option<usize>,
        #[clap(short, long)]
        from: Option<String>,
        #[clap(short, long)]
        to: Option<String>,
        #[clap(long)]
        tags: Vec<String>,
    },
    Edit {
        id: usize,
    },
    Search {
        query: String,
        #[clap(short, long, num_args = 1.., value_delimiter = ' ')]
        tags: Vec<String>,
    },
}

pub fn run() -> JotResult<()> {
    let cli = Cli::parse();
    let mut journal = storage::load_journal()?;

    match cli.command {
        Commands::Init => commands::init::execute(),
        Commands::Add { entry } => commands::add::execute(&mut journal, entry),
        Commands::Remove { id, from, to } => commands::remove::execute(&mut journal, id, from, to),
        Commands::View { id, from, to, tags } => {
            commands::view::execute(&journal, id, from, to, tags)
        }
        Commands::Edit { id } => commands::edit::execute(&mut journal, id),
        Commands::Search { query, tags } => commands::search::execute(&journal, &query, tags),
    }
}
