// Exercise 1 - Smart pointers as `Box<T>`
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>), // Indirect reference of a pointer to the next list.
    Nil,
}

fn create_empty_list() -> List {
    List::Nil
}

fn create_non_empty_list() -> List {
    List::Cons(2, Box::new(List::Cons(1, Box::new(List::Nil))))
}

fn main() {
    // Exercise 1:
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1:
    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
