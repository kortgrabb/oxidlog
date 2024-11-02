use storage::load_config;

mod cli;
mod commands;
mod error;
mod storage;
mod utils;

fn main() {
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        }
    };

    match cli::run(&config) {
        Ok(_) => {}
        Err(e) => {
            // print error kind and message
            eprintln!("{}", e);
        }
    }
}
