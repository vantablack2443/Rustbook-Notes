use std::env;

use std::process;
use minigrep::Config;

fn main() {

    // getting the arguments from command line
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        eprint!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    println!("Searching for : {}", config.query);
    println!("In file : {}", config.filename);



    // reading the content of the file and searching for the query in the content
    if let Err(e) =minigrep::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
