use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? operator returns the error value from the current function for the caller to handle
    // If the value is an Ok value, the Ok will be returned from this expression and the program will continue
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{}", contents);
    // need to wrap the OK return value in a Result, Ok, to match the signature of the run function
    Ok(())
}

