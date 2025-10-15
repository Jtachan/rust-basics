// https://github.com/rust-lang/rustlings/blob/main/exercises/03_if

fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if a > b { a } else { b }
}
fn animal_habitat(animal: &str) -> &str {
    /* TODO: Fix the compiler error in the statement below.
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2.0
    } else if animal == "snake" {
        3
    } else {
        "Unknown"
    };
    */

    // SOLUTION: all the return values must be the same type.
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        4
    };

    // Don't change the expression below!
    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}

fn main() {
    // Test: Bigger
    assert_eq!(10, bigger(10, 8));
    assert_eq!(42, bigger(32, 42));
    assert_eq!(42, bigger(42, 42));
    // Test: Animal Habitat
    assert_eq!(animal_habitat("gopher"), "Burrow");
    assert_eq!(animal_habitat("snake"), "Desert");
    assert_eq!(animal_habitat("crab"), "Beach");
    assert_eq!(animal_habitat("dinosaur"), "Unknown");

    println!("All tests passed!");
}
