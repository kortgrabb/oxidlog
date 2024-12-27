use crate::error::JotResult;
use crate::storage::config::Config;
use crate::{commands, storage};
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// New argument structs
#[derive(clap::Args, Clone)]
pub struct RemoveArgs {
    #[clap(short, long)]
    pub id: Option<usize>,
    #[clap(short, long)]
    pub range: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(clap::Args, Clone)]
pub struct ViewArgs {
    pub id: Option<usize>,
    #[clap(short, long)]
    pub from: Option<String>,
    #[clap(short, long)]
    pub to: Option<String>,
    #[clap(long)]
    pub tags: Vec<String>,
    #[clap(short, long)]
    pub recent: bool,
}

#[derive(clap::Args, Clone)]
pub struct SearchArgs {
    pub query: String,
    pub tags: Vec<String>,
    #[clap(short, long)]
    pub from: Option<String>,
    #[clap(short, long)]
    pub to: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add {
        entry: String,
    },
    Remove {
        #[clap(flatten)]
        args: RemoveArgs,
    },
    View {
        #[clap(flatten)]
        args: ViewArgs,
    },
    Edit {
        id: usize,
    },
    Search {
        #[clap(flatten)]
        args: SearchArgs,
    },
}

pub fn run(config: &Config) -> JotResult<()> {
    let cli = Cli::parse();
    let mut journal = storage::load_journal();

    match cli.command {
        Commands::Init => commands::init::execute(),
        Commands::Add { entry } => commands::add::execute(&mut journal, entry, config),
        Commands::Remove { args } => commands::remove::execute(&mut journal, args),
        Commands::View { args } => commands::view::execute(&journal, args, config),
        Commands::Edit { id } => commands::edit::execute(&mut journal, id),
        Commands::Search { args } => commands::search::execute(&journal, args, config),
    }
}
