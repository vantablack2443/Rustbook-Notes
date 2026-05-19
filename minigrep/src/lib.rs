use std::error::Error;
use std::fs;


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;
    // .expect("Something went wrong reading the file");
    // println!("Arguments: {:?}", args);
    // println!("Content: \n{}", content);

    for line in search(&config.query, &content){
        println!("{}", line);
    }
    
    Ok(())

}

pub struct Config{
    pub query: String,
    pub filename: String
}

impl Config{
    pub fn new(args:&[String]) -> Result<Config, &str>{
        if args.len() < 3 {
            return Err("Not enough arguments\n");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{query, filename})
    }
}


pub fn search <'a>(query: &str, content: &'a str) -> Vec<&'a str>{

    let mut results = Vec::new();

    for line in content.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "help";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
help me mister damu.";
        assert_eq!(vec!["help me mister damu."], search(query, content));
    }
}


