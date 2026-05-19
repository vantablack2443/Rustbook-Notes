use std::error::Error;
use std::fs;


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;
    // .expect("Something went wrong reading the file");
    // println!("Arguments: {:?}", args);
    println!("Content: \n{}", content);
    
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

