use std::env; // envoirment
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem with parsing arguments: {}", err); // eprint is to print to the standard erro stream
        process::exit(0); // stop the process when aruguments are wrong, code 0 means successfull exit
    });

    println!("");
    println!("Searching for: {}", config.query);
    println!("In File: {}", config.filename);
    println!("");

    // if run() returns an error do the block
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(0);
    }
}
