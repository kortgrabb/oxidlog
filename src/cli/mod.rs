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
    Add { entry: String },
    Remove { id: usize },
    View,
    Edit { id: usize },
}

pub fn run() -> JotResult<()> {
    let cli = Cli::parse();
    let mut journal = storage::load_journal()?;

    match cli.command {
        Commands::Init => commands::init::execute(),
        Commands::Add { entry } => commands::add::execute(&mut journal, entry),
        Commands::Remove { id } => commands::remove::execute(&mut journal, id),
        Commands::View => commands::view::execute(&journal),
        Commands::Edit { id } => commands::edit::execute(&mut journal, id),
    }
}
