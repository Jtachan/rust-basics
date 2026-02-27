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
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for '{}' in the file contents.", config.query);
    println!("Reading '{}'...\n", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
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
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Removing the first argument, which corresponds to the path to the executable.
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing the first argument: query."),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing second argument: file_path."),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
