use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("\nSearching for {} in file {}\n", config.query, config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? operator returns the error value from the current function for the caller to handle
      // If the value is an Ok value, the Ok will be returned from this expression and the program will continue
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{}", contents);
    // need to wrap the OK return value in a Result, Ok, to match the signature of the run function
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config {query, file_path})
    }
}
