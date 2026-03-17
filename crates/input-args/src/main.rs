//! `input-args` is an educational-purpose only tool, which main goal is to work as an ArgumentParser.
//! The concept of the tool is to:
//!
//! 1. Define a set of parameters that the user can provide (with defined types).
//! 2. Fetch and analyze any other parsed parameter that the user provides.
//!
//! Note: this project is NOT an alternative to [`clap`](https://docs.rs/clap/latest/clap/).

use std::{env, process};

#[derive(Debug)]
struct AppOptions {
    target_path: String,
    verbose: bool,
    max_value: Option<usize>,
    others: Vec<String>,
}

impl AppOptions {
    fn display(&self) {
        let verbose_level = { if self.verbose { "ON" } else { "OFF" } };
        let value = {
            if self.max_value == None {
                String::from("Not specified")
            } else {
                self.max_value.unwrap().to_string()
            }
        };

        println!("===== Provided arguments =====");
        println!("Target Path : {}", self.target_path);
        println!("Verbosity   : {verbose_level}");
        println!("Value       : {value}");

        if self.others.len() > 0 {
            println!("Other provided arguments:");
            for arg in self.others.iter() {
                println!("  {}", arg);
            }
        }
    }
}

fn parse_args() -> AppOptions {
    let args: Vec<String> = env::args().collect();

    // Options
    let mut target_path: Option<String> = None;
    let mut verbose = true;
    let mut max_value: Option<usize> = None;
    let mut others: Vec<String> = Vec::new();

    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--quiet" | "-q" => {
                verbose = false;
                i += 1;
            }
            "--target" | "-t" => {
                target_path = Some(args[i + 1].to_string());
                i += 2
            }
            "--max-value" | "-m" => {
                max_value = Some(args[i + 1].parse::<usize>().unwrap_or_else(|e| {
                    panic!(
                        "Could not correctly convert the argument 'max-value' due to an error: {}",
                        e
                    )
                }));
                i += 2;
            }
            _ => {
                let mut other_arg = args[i].clone();
                i += 1;
                while i < args.len() && !args[i].starts_with("-") {
                    other_arg.push(' ');
                    other_arg.push_str(args[i].as_str());
                    i += 1;
                }
                others.push(other_arg);
            }
        }
    }

    if target_path == None {
        eprintln!("No target path was provided.");
        process::exit(1);
    }

    AppOptions {
        target_path: target_path.unwrap(),
        verbose,
        max_value,
        others,
    }
}

fn main() {
    let arguments = parse_args();
    arguments.display();
}
