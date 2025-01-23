use crate::commands::{backup, init};
use crate::error::{JotError, JotResult};
use crate::storage::config::Config;
use crate::{commands, storage};
use clap::{Parser, Subcommand};

use commands::{add, edit, export, remove, search, view};

/// A command-line journaling tool for quick note-taking and organization
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new journal or reconfigure an existing one
    #[command(visible_alias = "i")]
    Init {
        #[clap(flatten)]
        args: init::InitArgs,
    },

    /// Add a new entry to your journal
    #[command(visible_alias = "new", visible_alias = "a")]
    Add {
        #[clap(flatten)]
        args: add::AddArgs,
    },

    /// Remove an entry from your journal
    #[command(visible_alias = "delete", visible_alias = "rm")]
    Remove {
        #[clap(flatten)]
        args: remove::RemoveArgs,
    },

    /// View journal entries
    #[command(visible_alias = "list", visible_alias = "ls")]
    View {
        #[clap(flatten)]
        args: view::ViewArgs,
    },

    /// Edit an existing journal entry
    #[command(visible_alias = "modify", visible_alias = "e")]
    Edit {
        #[clap(flatten)]
        args: edit::EditArgs,
    },

    /// Search through journal entries
    #[command(visible_alias = "find", visible_alias = "s")]
    Search {
        #[clap(flatten)]
        args: search::SearchArgs,
    },

    /// Export journal entries to various formats
    #[command(visible_alias = "dump", visible_alias = "ex")]
    Export {
        #[clap(flatten)]
        args: export::ExportArgs,
    },

    #[command(visible_alias = "b")]
    Backup {
        #[clap(flatten)]
        args: backup::BackupArgs,
    },
}

/// Runs a single command based on the parsed CLI arguments
///
/// # Arguments
///
/// * `config` - Configuration settings loaded from the config file
///
/// # Returns
///
/// Returns `JotResult<()>` indicating success or error
///
/// # Errors
///
/// Will return a `JotError` if:
/// * Journal loading fails
/// * Any command-specific execution errors occur
pub fn run(config: &Config) -> JotResult<()> {
    let cli = Cli::parse();

    // Only load journal for commands that need it
    match cli.command {
        Commands::Init { args } => commands::init::execute(args),
        Commands::Add { args } => {
            let mut journal = storage::load_journal()?;
            commands::add::execute(&mut journal, args, config)
        }
        Commands::Remove { args } => {
            let mut journal = storage::load_journal()?;
            commands::remove::execute(&mut journal, args)
        }
        Commands::View { args } => {
            let journal = storage::load_journal()?;
            commands::view::execute(&journal, args, config)
        }
        Commands::Edit { args } => {
            let mut journal = storage::load_journal()?;
            commands::edit::execute(&mut journal, args)
        }
        Commands::Search { args } => {
            let journal = storage::load_journal()?;
            commands::search::execute(&journal, args, config)
        }
        Commands::Export { args } => {
            let journal = storage::load_journal()?;
            commands::export::execute(&journal, args, config)
        }
        Commands::Backup { args } => {
            let mut journal = storage::load_journal()?;
            commands::backup::execute(&mut journal, args)
        }
    }
}
