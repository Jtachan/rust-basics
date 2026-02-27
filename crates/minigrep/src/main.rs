/*
   MINIGREP: An I/O Project from the Rust Book (chapter 12)
       https://doc.rust-lang.org/book/ch12-00-an-io-project.html

   Features
   --------
   CLI arguments
       By running `cargo run --` anything provided after `--` is considered an argument.
*/
use minigrep::{search, search_case_insensitive};
use std::error::Error;
use std::{env, fs, process};

fn main() {
    // The first argument is the path to the executable (target folder).
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for '{}' in the file contents.", config.query);
    println!("Reading '{}'...\n", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(2);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    if results.len() == 0 {
        println!("No matches found");
    }
    for line in results {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
