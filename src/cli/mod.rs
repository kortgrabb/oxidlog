use crate::commands::init;
use crate::error::JotResult;
use crate::storage::config::Config;
use crate::{commands, storage};
use clap::{Parser, Subcommand};

use commands::{add, edit, export, remove, search, view};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[clap(flatten)]
        args: init::InitArgs,
    },
    #[command(alias = "new")]
    Add {
        #[clap(flatten)]
        args: add::AddArgs,
    },
    #[command(alias = "delete")]
    Remove {
        #[clap(flatten)]
        args: remove::RemoveArgs,
    },
    #[command(alias = "list")]
    View {
        #[clap(flatten)]
        args: view::ViewArgs,
    },
    #[command(alias = "modify")]
    Edit {
        #[clap(flatten)]
        args: edit::EditArgs,
    },
    #[command(alias = "find")]
    Search {
        #[clap(flatten)]
        args: search::SearchArgs,
    },
    #[command(alias = "dump")]
    Export {
        #[clap(flatten)]
        args: export::ExportArgs,
    },
}

pub fn run(config: &Config) -> JotResult<()> {
    let cli = Cli::parse();
    let mut journal = storage::load_journal();

    match cli.command {
        Commands::Init { args } => commands::init::execute(args),
        Commands::Add { args } => commands::add::execute(&mut journal, args, config),
        Commands::Remove { args } => commands::remove::execute(&mut journal, args),
        Commands::View { args } => commands::view::execute(&journal, args, config),
        Commands::Edit { args } => commands::edit::execute(&mut journal, args),
        Commands::Search { args } => commands::search::execute(&journal, args, config),
        Commands::Export { args } => commands::export::execute(&mut journal, args, config),
    }
}
