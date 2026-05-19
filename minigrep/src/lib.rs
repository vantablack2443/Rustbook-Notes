use std::error::Error;
use std::fs;
use std::env;

// run function to read the content of the file and search for the query in the content
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;
    // .expect("Something went wrong reading the file");
    // println!("Arguments: {:?}", args);
    // println!("Content: \n{}", content);

    for line in if config.case_sensitive {
        search_case_sensitive(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    } {
        println!("{}", line);
    }
    
    Ok(())

}

// Config struct to hold the query and filename
pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

// implementation of the Config struct to create a new Config instance from the command line arguments
impl Config{
    pub fn new(args:&[String]) -> Result<Config, &str>{
        if args.len() < 3 {
            return Err("Not enough arguments\n");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{query, filename, case_sensitive})
    }
}


// function to search for the query in the content in a case sensitive way

pub fn search_case_sensitive <'a>(query: &str, content: &'a str) -> Vec<&'a str>{

    let mut results = Vec::new();

    for line in content.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}


// function to search for the query in the content in a case insensitive way
pub fn search_case_insensitive <'a>(query: &str, content: &'a str) -> Vec<&'a str>{

    let mut results = Vec::new();

    for line in content.lines(){
        if line.to_lowercase().contains(&query.to_lowercase()){
            results.push(line);
        }
    }
    results
}


// tests for the search functions
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "help";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
help me mister damu.";
        assert_eq!(vec!["help me mister damu."], 
        search_case_sensitive(query, content));
    }


    #[test]
    fn case_insensitive(){
        let query = "DuCt";
        let content = "\
Rust is DuCt:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["Rust is DuCt:", "Duct tape.", "safe, fast, productive."].sort(),
        search_case_insensitive(query, content).sort());
    }
}


