// Exercise 1: Fix the signature
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Exercise 2: Correct call of lifetimes
fn evaluate_chains() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        // The print command must be invoked while 'str2' is still on memory
        println!("The longest string is '{result}'");
    }
}

// Exercise 3: Lifetimes in structures:
// It requires lifetime to guarantee the data won't be dropped while the struct still exists
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    // Exercise 1:
    assert_eq!(longest("abcd", "123"), "abcd");
    assert_eq!(longest("abc", "1234"), "1234");

    // Exercise 2:
    evaluate_chains();

    // Exercise 3:
    let book = Book{
        author: "George Orwell",
        title: "1984",
    };
    println!("{} by {}", book.title, book.author);

    println!("All tests passed");
}
