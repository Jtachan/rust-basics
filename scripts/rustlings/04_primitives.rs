// https://github.com/rust-lang/rustlings/blob/main/exercises/04_primitive_types

fn greetings(is_morning: bool) -> String {
    /*  EXPLANATION NOTE: 'return' statement.
           In rust, everything is an expression. At previous exercises there were
           expressions like 'let x = if cond { 1 } else { 2 }' that had an
           "implicit return", as the whole expression was a statement.

           As the following example requires a control flow which is not immediately
           a return statement, the 'return' is explicitly needed here.
    */

    if is_morning {
        return "Good morning!".to_string();
    }

    // TODO: Define a boolean variable with the name `is_evening` before the `if` statement below.
    //  The value of the variable should be the negation (opposite) of `is_morning`.
    let is_evening = !is_morning;
    if is_evening {
        return "Good evening!".to_string();
    }

    // Code should not reach here, this is just the implicit return to satisfy the compiler.
    "Good day!".to_string()
}

fn character_check(your_char: char) -> String {
    // Note: chars are defined with single quotes.
    if your_char.is_alphabetic() {
        "Alphabetical!".to_string()
    } else if your_char.is_numeric() {
        "Numerical!".to_string()
    } else {
        "Neither alphabetic nor numeric!".to_string()
    }
}

fn create_array(a: &[i32]) -> String {
    // TODO: Create an array called `a` with at least 100 elements in it.
    /* Note:
        The syntaxis "let a = [1; nof_elem];" allows creating an array of as many elements as specified.
        However, this won't compile. 'nof_elem' must be a known value when creating an array.

        For now, consider that only an array could be parsed into the function.
     */
    if a.len() >= 100 {
        "That is a big array!".to_string()
    } else {
        "The array is not big enough!".to_string()
    }
}

fn rust_slices() {
    // Array
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..4];
    assert_eq!([2, 3, 4], slice);
    // String
    let a = "Hello world!";
    let slice = &a[..5];
    assert_eq!("Hello", slice);
    let slice = &a[6..11];
    assert_eq!("world", slice);
}

fn main() {
    // Test: Greetings (booleans)
    let is_morning: bool = true;
    assert_eq!(greetings(is_morning), "Good morning!");
    let is_morning: bool = false;
    assert_eq!(greetings(is_morning), "Good evening!");
    // Test: Character check
    let my_char: char = 'C';
    assert_eq!(character_check(my_char), "Alphabetical!");
    let my_char: char = '4';
    assert_eq!(character_check(my_char), "Numerical!");
    let my_char: char = '😉';
    assert_eq!(character_check(my_char), "Neither alphabetic nor numeric!");
    // Test: Create array
    let a = [1, 5];
    assert_eq!(create_array(&a), "The array is not big enough!");
    let a: [i32; 100] = [5; 100];
    assert_eq!(create_array(&a), "That is a big array!");
    // Test: slices
    rust_slices();
    // Test: Unpacking tuples
    let cat = ("Cookie", 1.7);
    let (name, age) = cat;
    assert_eq!(name, "Cookie");
    assert_eq!(age, 1.7);
    // Test: indexing
    let numbers = (1, 2, 3);
    let second = numbers.1;
    assert_eq!(second, 2, "This is not the 2nd number!");
    let numbers = [1, 2, 3];
    let second = numbers[1];
    assert_eq!(second, 2, "This is not the 2nd number!");

    println!("All tests passed!");
}
