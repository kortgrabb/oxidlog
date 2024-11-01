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
    },
    View {
        #[clap(short, long)]
        id: Option<usize>,
        #[clap(short, long)]
        from: Option<String>,
        #[clap(short, long)]
        to: Option<String>,
    },
    Edit {
        id: usize,
    },
    Search {
        query: String,
    },
}

pub fn run() -> JotResult<()> {
    let cli = Cli::parse();
    let mut journal = storage::load_journal()?;

    match cli.command {
        Commands::Init => commands::init::execute(),
        Commands::Add { entry } => commands::add::execute(&mut journal, entry),
        Commands::Remove { id } => commands::remove::execute(&mut journal, id),
        Commands::View { id, from, to } => commands::view::execute(&journal, id, from, to),
        Commands::Edit { id } => commands::edit::execute(&mut journal, id),
        Commands::Search { query } => commands::search::execute(&journal, &query),
    }
}
