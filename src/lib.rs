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
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str>{
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file_path"),
        };

        let sensitive_flag = match args.next() {
            Some(arg ) => if arg == "-i" { true } else { false },
            None => false,
        };

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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// search a lowercase word in a lowercase content
fn search_no_sensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str>{
    contents
        .lines()
        .filter(
            |line| 
            line
                .to_lowercase()
                .contains(&query.to_lowercase())
        )
        .collect()
}
