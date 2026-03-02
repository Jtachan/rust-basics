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

/// Defines the Fibonacci Series with length `n`.
///
/// # Examples
/// ```rust
/// assert_eq!(fibonacci(0), []);
/// assert_eq!(fibonacci(3), [0, 1, 1]);
///
/// let series: Vec<u64> = fibonacci(17);
/// assert_eq!(series.len(), 17);
/// assert_eq!(series.pop(), 987);
/// ```
pub fn fibonacci(n: usize) -> Vec<u64> {
    let mut sequence: Vec<u64> = Vec::new();
    for i in 1..=n {
        if i < 3 {
            sequence.push((i - 1) as u64);
        } else {
            sequence.push(sequence[i - 2] + sequence[i - 3]);
        }
    }
    sequence
}

#[cfg(test)]
mod tests {
    use std::intrinsics::assert_inhabited;
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn fib_series_initialization() {
        assert_eq!(fibonacci(0), []);
        assert_eq!(fibonacci(2), [0, 1]);
        assert_eq!()
    }

    #[test]
    fn fib_long_series() {
        let mut series = fibonacci(17);
        assert_eq!(series.len(), 17);
        assert_eq!(series.pop(), Some(987));
        let mut series = fibonacci(17);
        assert_eq!(series.len(), 17);
        assert_eq!(series.pop(), Some(987));
    }
}
