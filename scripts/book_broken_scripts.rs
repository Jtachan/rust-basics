/*
    Here are contained multiple scripts from the Rust Interactive Book (rust-book.cs.brown.edu)
    which will NOT compile.
    This file is an extra help to test the non-compiling cases and learn better.
 */


fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    // 'dst' has read & write permissions (borrow)

    let largest: &String =
        // 'dst' returns the write permission
        dst.iter().max_by_key(|s| s.len()).unwrap();

    for s in src {
        if s.len() > largest.len() {
            // 'dst' requires of write permissions
            dst.push(s.clone());
        }
    }
}