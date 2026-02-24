fn first_exercise() {
    /*
        Solution: It is required to define the type of the items within the vector.
        In this case, as the 'into()' call is being used, not all types are valid for the
        items in the vector. Type 'i16' is the smallest allowed.
    */
    let mut numbers: Vec<i16> = Vec::new();

    // Don't change these lines below
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}

/* Second exercise from here:
   Do the following at the exercise:
       1. Rewrite the struct to support any type
       2. Adapt the implementation to be generic over the wrapped value
*/
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    first_exercise();

    // Exercise 2:
    assert_eq!(Wrapper::new(42).value, 42);
    assert_eq!(Wrapper::new("Foo").value, "Foo");

    println!("All tests passed!");
}
