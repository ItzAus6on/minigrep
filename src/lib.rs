use std::error::Error;
use std::fs;

// Configuation
// Strore the arguments from command line
pub struct Config {
    // word that need to find
    query: String,
    // the original text file
    file_path: String,
    // flag that judge whether need capital or not
    sensitive_flag: bool,
}

impl Config {
    // create and initialize a configuratoion
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        // if the number of the argument is smaller than 3
        // return Not enough arguments
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        
        // copy the configuration content
        let query = args[1].clone();
        let file_path = args[2].clone();
        // if the number of the argument reaches 4
        // and the last argument is "-i" return true
        let sensitive_flag = if args.len() == 4 && args[3] == "-i" {true} else {false};

        // return the configuration
        Ok( Config {query, file_path, sensitive_flag} )
    }
}

// the main body of the code
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read the contents 
    // ? means return error msg while error occurs
    let contents = fs::read_to_string(config.file_path)?;
    
    // create "results" to receieve the final search results 
    // if the flag is true use the modified one
    // vice versa
    let results = if config.sensitive_flag {
        search_no_sensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    // print all the lines in "results", which has the required word
    for line in results {
        println!("{}", line);
    }

    // if there is no problem, return ok
    Ok(())
}

// search a normal word in a normal content
fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str>{
    // create "results" to receieve final search results
    let mut results = Vec::new();

    // if the word is found, push the line into "results"
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    // return the final answer
    results
}

// search a lowercase word in a lowercase content
fn search_no_sensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str>{
    // cover the original string to a lowercase one
    let query = query.to_lowercase();
    // create "results" to receieve final search results
    let mut results = Vec::new();

    // if the word is found, push the line into "results"
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    // return the final answer
    results
}
