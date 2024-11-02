mod cli;
mod commands;
mod error;
mod storage;
mod utils;

fn main() {
    match cli::run() {
        Ok(_) => {}
        Err(e) => {
            // print error kind and message
            eprintln!("{}", e);
        }
    }
}
