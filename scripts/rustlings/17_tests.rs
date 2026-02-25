/*
   RUNNING TESTS:
   Compile the file with the `--test` flag, then run the generated executable.
   Note: the compiler won't complain if the main is empty when the `--test` flag is provided.
*/

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to `pow(2, n)`.
fn power_of_2(n: u8) -> u64 {
    1 << n
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height must be positive");
        }
        Rectangle { width, height }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_is_even() {
        // Testing the function 'is_even' with some values
        assert!(is_even(42));
        assert!(!is_even(75));
    }

    #[test]
    fn ex2_power2() {
        // Asserting equal values
        assert_eq!(power_of_2(0), 1);
        assert_eq!(power_of_2(2), 4);
        assert_eq!(power_of_2(5), 32);
        assert_eq!(power_of_2(25), 33554432);
    }

    #[test]
    fn ex3_correct_rect() {
        // Comparing values directly
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20);
    }

    #[test]
    #[should_panic]
    fn ex3_negative_width() {
        // Checking the code panics
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic]
    fn ex3_negative_height() {
        // Checking the code panics
        let _rect = Rectangle::new(10, -10);
    }

    #[test]
    #[ignore]
    fn ex4_ignore_test() {
        panic!("Test wasn't ignored");
    }
}
