extern crate rust_grep;

use rust_grep::Config;

use std::env;
use std::process;
use std::io::prelude::*;

fn main() {
    let mut stderr = std::io::stderr();
    
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        writeln!(
            &mut stderr,
            "Problem parsing arguments: {}",
            err).expect("Could not write to stderr");
            
        process::exit(1)
    });

    println!("Searching for {} in file {}.", config.search, config.filename);

    if let Err(e) = rust_grep::run(config) {
        writeln!(
            &mut stderr,
            "Application error: {}",
            e).expect("Could not write to stderr");
        process::exit(1)
    }
}
