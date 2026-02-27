/*
   MINIGREP: An I/O Project from the Rust Book (chapter 12)
       https://doc.rust-lang.org/book/ch12-00-an-io-project.html

   Features
   --------
   CLI arguments
       By running `cargo run --` anything provided after `--` is considered an argument.
*/
use std::env;
use std::fs;

fn main() {
    // The first argument is the path to the executable (target folder).
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("Reading '{file_path}'...\n");
    let contents = fs::read_to_string(file_path).expect("Unable to read the file");

    println!("Read content:\n-------------\n{contents}");
}
