use std::env; // envoirment
use std::process;

// use minigrep::Config;   // if the file is named lib.rs   (libraty crate, I guess, maybe)
// mod thing; // if the file is names not lib.rs   (auto into -> module)

// if more declare shit here
mod thing {
    pub mod thing;
    // pub mod subfolder {   // if there was a folder in the folder
    //     pub mob subfile;
    // }
}

fn main() {
    let config = thing::thing::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem with parsing arguments: {}", err); // eprint is to print to the standard erro stream
        process::exit(0); // stop the process when aruguments are wrong, code 0 means successfull exit
    });

    println!("");
    println!("Searching for: {}", config.query);
    println!("In File: {}", config.filename);
    println!("");

    // if run() returns an error do the block
    if let Err(e) = thing::thing::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(0);
    }
}
