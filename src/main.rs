mod cli;
mod commands;
mod error;
mod storage;
mod utils;

fn main() {
    match cli::run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
