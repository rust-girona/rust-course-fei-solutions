//! Run this file with `cargo test --test 07_adjacent_diff`.

//! TODO: Implement a function called `adjacent_diff`, which will receive a slice of numbers, and it
//! will return an Iterator over the differences of adjacent numbers.
//! E.g. `adjacent_diff(&[1, 2, 4, 1])` will iterate `1`, `2`, and `-3`.
//!
//! Try to implement the iterator directly within the function, using various iterator combinators.
//! Do not create a separate struct that would implement the `Iterator` trait.
//!
//! The iterator has to be **lazy**! It should not copy the whole input array
//! (in other words, it should have space complexity O(1)).

use std::iter::Iterator;

fn adjacent_diff(numbers: &[i64]) -> impl Iterator<Item = i64> + '_ {
    numbers.windows(2).map(|pair| pair[1] - pair[0])
}

/*
 * NOTES of my way to get the solution.
 *
 * I started with this approach. This didn't compile and I didn't understand the compiler error
 * message.
 * I asked AI, but I forgot to mention to ONLY explain the problem without giving me the solution,
 * so he presented the implemented solution.
 *
 * After I asked, "but, could it be done with my approach?" it gave me the next commented solution.
 *
 * I still didn't understand why this solution couldn't compile and in the end, I remember that in
 * Rust each closure is a different anonymous type, so the return types from the `else` branch and
 * the one at the end of the function are different, so it doesn't compile.
 *
fn adjacent_diff(numbers: &[i64]) -> impl Iterator<Item = i64> + '_ {
    let mut it = numbers.iter().copied();
    let mut prev = if let Some(n) = it.next() {
        n
    } else {
        return it
    };

    it.map(|n| {
        let mapped = n - prev;
        prev = n;
        mapped
    })
}

fn adjacent_diff(numbers: &[i64]) -> impl Iterator<Item = i64> + '_ {
    let mut it = numbers.iter().copied();
    let mut prev = it.next();

    it.filter_map(move |n| prev.replace(n).map(|p| n - p))
*/

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::adjacent_diff;

    #[test]
    fn empty() {
        assert_eq!(adjacent_diff(&[]).count(), 0);
    }

    #[test]
    fn single_item() {
        assert_eq!(adjacent_diff(&[1]).count(), 0);
    }

    #[test]
    fn two_items() {
        assert_eq!(adjacent_diff(&[1, 3]).collect::<Vec<_>>(), vec![2]);
    }

    #[test]
    fn many_items() {
        assert_eq!(
            adjacent_diff(&[1, 3, 2, 4, 8, 12, 5, 10]).collect::<Vec<_>>(),
            vec![2, -1, 2, 4, 4, -7, 5]
        );
    }
}
