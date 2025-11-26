// Exercise 1: Fixing the function without modifying the signature.
fn current_favourite_color() -> String {
    "blue".to_string()
}

// Exercise 2: Fix compiler error
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

// Exercise 3: Complete the functions
fn trim_me(input: &str) -> &str {
    // Removing whitespaces from both ends of a string.
    input.trim()
}

fn compose_me_1(input: &str) -> String {
    // Add " world!" to the string.
    // Solution 1:
    let mut s = String::from(input);
    s.push_str(" world!");
    s
}

fn compose_me_2(input: &str) -> String {
    // Add " world!" to the string.
    // Solution 2:
    String::from(input) + " world!"
}


fn replace_me(input: &str) -> String {
    // Replace "cars" with "balloons"
    let s = input.replace("cars", "balloons");
    s.to_string()
}

// Exercise 4: Identification of &str and String
fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

fn main() {
    // Exercise 1
    let answer = current_favourite_color();
    println!("My current favourite color is {answer}");

    // Exercise 2
    let word = String::from("green");
    assert!(is_a_color_word(&word));
    let word = String::from("Spain");
    assert!(!is_a_color_word(&word));

    // Exercise 3
    assert_eq!(trim_me("Hello!    "), "Hello!");
    assert_eq!(trim_me("  What's up!"), "What's up!");
    assert_eq!(trim_me("   Hola!  "), "Hola!");
    assert_eq!(trim_me("Hi!"), "Hi!");

    assert_eq!(compose_me_1("Hello"), "Hello world!");
    assert_eq!(compose_me_2("Goodbye"), "Goodbye world!");

    assert_eq!(
        replace_me("I think cars are cool"),
        "I think balloons are cool",
    );
    assert_eq!(
        replace_me("I love to look at cars"),
        "I love to look at balloons",
    );

    // Exercise 4
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());

    println!("All test passed!");
}
