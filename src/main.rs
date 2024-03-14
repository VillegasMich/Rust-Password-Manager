mod cli;
mod file;
mod handlers;
mod models;
mod utils;
use std::io;

fn main() -> io::Result<()> {
    cli::parse()
}
