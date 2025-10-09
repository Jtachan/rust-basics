/*
    Here are contained multiple scripts from the Rust Interactive Book (rust-book.cs.brown.edu)
    which will NOT compile.
    This file is an extra help to test the non-compiling cases and learn better.
 */
#![allow(unused)]

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    // The next commented code is the one that won't allow to compile:

    // let largest: &String =
    //     // 'dst' borrows ownership permission to 'largest'
    //     dst.iter().max_by_key(|s| s.len()).unwrap();
    //
    // for s in src {
    //     if s.len() > largest.len() {
    //         // 'dst' requires of write permissions
    //         dst.push(s.clone());
    //     }
    // }

    // FIX 1: use .clone() to define largest and not have permission conflicts
    //      let largest = dst.[...].unwrap().clone();
    // this will however may create performance issues.

    // FIX 2: As the code only needs the length of 'largest', just save that value.
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            // 'dst' requires of write permissions
            dst.push(s.clone());
        }
    }
}

fn main() {
    // Defining a vector containing one element String
    let mut v: Vec<String> = vec![String::from("Hello world")];
    // Here the string is moved out of the vector.
    let mut s: String = v.remove(0);
    // Adding '!' at the end of the string.
    s.push('!');
    println!("{s}");
    // Asserting now the vector is empty.
    assert!(v.len() == 0);
}