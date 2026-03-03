//! Math functions that can be simplified with iterators.

/* This is just a small exercise to get used to Rust iterators and function documentation.
    All documentation is deployable via `rustdoc math_iter.rs`. Then the index is at
    `doc/math_iter/index.html`.

    The documentation comments use markdown for formatting.
 */

use std::iter;

/// Obtains the factorial of the number `num` (unsigned).
/// The factorial of any number is defined as:
///
/// - 1 if the number is 0.
/// - The multiplication of all numbers from 1 up to the number for any number equal or higher to 0.
///
/// For more information, see <https://en.wikipedia.org/wiki/Factorial>
pub fn factorial(num: u64) -> u64 {

    // INTUITIVE SOLUTION: for-loop
    // -------------------
    // let mut result = 1;
    // for n in 2..=num {
    //     result *= n;
    // }
    // result

    // SOLUTION: Using `fold` -> https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
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

    // INTUITIVE SOLUTION: Using a for-loop
    // ------------------------------------
    // let mut sequence: Vec<u64> = Vec::new();
    // for i in 1..=n {
    //     if i < 3 {
    //         sequence.push((i - 1) as u64);
    //     } else {
    //         sequence.push(sequence[i - 2] + sequence[i - 3]);
    //     }
    // }
    // sequence

    // SOLUTION: using iter::successors -> https://doc.rust-lang.org/std/iter/fn.successors.html
    iter::successors(Some((0u64, 1u64)), |(a, b)| Some((*b, a + b)))
        .map(|(a, _)| a)
        .take(n)
        .collect()
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

    #[test]
    fn fib_series_initialization() {
        assert_eq!(fibonacci(0), []);
        assert_eq!(fibonacci(2), [0, 1]);
    }

    #[test]
    fn fib_long_series() {
        let mut series = fibonacci(17);
        assert_eq!(series.len(), 17);
        assert_eq!(series.pop(), Some(987));
    }
}
