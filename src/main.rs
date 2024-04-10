use std::env;
use minigrep::Config;
use std::process;

fn main() {
    // input arguments from command line
    // create a configuration that receives arguments
    // if cannot parsing arguments during this time, exit the program
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // run the main body
    // if there is something wrong, return err value and print it
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
