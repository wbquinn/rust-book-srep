//! srep
//!
//! `srep` is a simple command line text searcher

use std::{env, process};

use srep::Config;
/// Usage
///
/// the program takes 2 arguments - query string and file path
///
/// e.g. `srep "text to search for" "path to file to search in"`
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    eprint!("Searching for \"{}\" ", config.query);
    eprintln!("in file \"{}\"", config.file_path);

    if let Err(e) = srep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
