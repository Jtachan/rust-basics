//! Math functions that can be simplified with iterators.

/* This is just a small exercise to get used to Rust iterators and function documentation.
    All documentation is deployable via `rustdoc math_iter.rs`. Then the index is at
    `doc/math_iter/index.html`.

    The documentation comments use markdown for formatting.
 */

/// Obtains the factorial of the number `num` (unsigned).
/// The factorial of any number is defined as:
///
/// - 1 if the number is 0.
/// - The multiplication of all numbers from 1 up to the number for any number equal or higher to 0.
///
/// For more information, see <https://en.wikipedia.org/wiki/Factorial>
pub fn factorial(num: u64) -> u64 {
    (2..=num).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(5), 120);
    }
}
