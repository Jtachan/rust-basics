fn main() {}
// ------------------------- Exercise 2 Block

/*
   Capitalize the first character of a single word.

   Example
   -------
   "hello" -> "Hello"
*/
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    let mut res = match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string(),
    };
    for char in chars {
        res.push(char);
    }

    res
}

/*
   Given an array of `&str`, capitalize the first character of each word.
   Returns a vector with the modified words.

   Example
   -------
   ["hello", "world"] -> ["Hello", "World"]
*/
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut result = Vec::new();

    for word in words {
        result.push(capitalize_first(word));
    }

    result
}

/*
   Given an array of `&str`, capitalize the first character of each word and
   joins all the words in a string.

   Example
   -------
   ["hello", " ", "world"] -> "Hello World"
*/
fn capitalize_words_string(words: &[&str]) -> String {
    let mut result = String::new();

    for word in words {
        result.push_str(&capitalize_first(word));
    }

    result
}

// ------------------------- Exercise 3 Block
#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    DivideByZero, // a / 0
    IntegerOverflow,
    NotDivisible, // Any result returning a float: 5 / 2 = 2.5
}

// Calculates a / b returning either the correct result or error.
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }
    if a == i64::MIN && b == -1 {
        return Err(DivisionError::IntegerOverflow);
    }
    if a % b != 0 {
        return Err(DivisionError::NotDivisible);
    }

    Ok(a / b)
}

fn result_with_list() -> Result<Vec<i64>, DivisionError> {
    let numbers = [27, 297, 38502, 81];
    let div_results = numbers.iter().map(|n| divide(*n, 27));
    div_results.collect()
}

fn list_of_results() -> Vec<Result<i64, DivisionError>> {
    let numbers = [27, 297, 38502, 81];
    let div_results = numbers.iter().map(|n| divide(*n, 27));
    div_results.collect()
}

// ------------------------- Exercise 4 Block
fn factorial(num: u64) -> u64 {
    // `(2..=num)` -> iterator from '2' to 'num' (lazy iterator, only run when called).
    // `fold(init, func)` -> see https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
    (2..=num).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_iterators() {
        // Create an iterator and go over all the items until exhausting it.
        let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];
        let mut fav_fruits_iterator = my_fav_fruits.iter();

        assert_eq!(fav_fruits_iterator.next(), Some(&"banana"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"custard apple"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"peach"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry"));
        assert_eq!(fav_fruits_iterator.next(), None);
    }

    #[test]
    fn ex2_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn ex2_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn ex2_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }

    #[test]
    fn ex3_success() {
        assert_eq!(divide(81, 9), Ok(9));
        assert_eq!(divide(81, -1), Ok(-81));
        assert_eq!(divide(i64::MIN, i64::MIN), Ok(1));
        assert_eq!(divide(0, 84), Ok(0));
    }

    #[test]
    fn ex3_errors() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn ex3_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn ex3_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }

    #[test]
    fn ex4_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(4), 24);
    }
}
