//! Run this file with `cargo test --test 05_bubble_sort`.

// TODO: Write a simple bubble sort implementation that receives a unique (mutable) reference
// to a slice of numbers and sorts them in-place.

fn bubble_sort(list: &mut [i64]) {
    if list.len() <= 1 {
        return;
    }

    let mut i = list.len() - 1;
    while i > 0 {
        let mut last_swap = i;
        for j in 0..i {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
                last_swap = j;
            }
        }

        if last_swap != i {
            i = last_swap;
        } else {
            // NOTE: Initially I had the following instruction instead of the `break` statement,
            // but because a question that @frankPairs dropped in his review, I realized that
            // having a `break` works and it is even optimal because it can save some iterations,
            // despite the computation complexity time of the algorithm won't change.
            // i -= 1;
            break;
        }
    }
}

/*
// I started with this implementation, although I got caught by Rust not doing reverse sequences
// with the common syntax, so `for i in 10..0` doesn't do anything that's caught by Clippy, but not
// `for i in list.len()..0`, which was the case for my initial implementation and I got crazy why
// the tests weren't passed, until I found out.

fn bubble(list: &mut [i64]) {
    if list.len() <= 1 {
        return;
    }

    for i in (0..list.len()).rev() {
        for j in 0..i {
            if list[j] > list[j + 1] {
                let tmp = list[j];
                list[j] = list[j + 1];
                list[j + 1] = tmp;
            }
        }
    }
}

// Clippy warned in the lines inside of the conditional of the inner loop about
// https://rust-lang.github.io/rust-clippy/rust-1.93.0/index.html#manual_swap
// It was a false positive because Rust doesn't allow to take 2 mutable references to the same
// slice. But slices has the `swap` method, so it can be used that way.
// After I wrote the above uncommented implementation which reduce the number of iteration in some
// cases, not always.

fn bubble_sort(list: &mut [i64]) {
    if list.len() <= 1 {
        return;
    }

    for i in (0..list.len()).rev() {
        for j in 0..i {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
}

*/

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::bubble_sort;
    use rand::seq::SliceRandom;

    #[test]
    fn empty() {
        test_sort(&mut [], &[]);
    }

    #[test]
    fn single_element() {
        test_sort(&mut [1], &[1]);
    }

    #[test]
    fn same_elements() {
        test_sort(&mut [1, 1, 1, 1, 1], &[1, 1, 1, 1, 1]);
    }

    #[test]
    fn already_in_order() {
        test_sort(&mut [1, 2, 3], &[1, 2, 3]);
    }

    #[test]
    fn already_in_order_long() {
        let mut data: Vec<i64> = (0..1000).collect();
        test_sort(&mut data, &(0..1000).collect::<Vec<_>>());
    }

    #[test]
    fn arbitrary() {
        test_sort(
            &mut [8, 1, -4, 1, 4, 4, 3, 2, 1, -5, 4, 1024, 8667, 10],
            &[-5, -4, 1, 1, 1, 2, 3, 4, 4, 4, 8, 10, 1024, 8667],
        );
    }

    #[test]
    fn reverse_order() {
        test_sort(&mut [3, 2, 1], &[1, 2, 3]);
    }

    #[test]
    fn reverse_order_long() {
        let mut data: Vec<i64> = (0..1000).rev().collect();
        test_sort(&mut data, &(0..1000).collect::<Vec<_>>());
    }

    #[test]
    fn shuffle() {
        let mut data: Vec<_> = (0..1000).collect();
        data.shuffle(&mut rand::rng());

        test_sort(&mut data, &(0..1000).collect::<Vec<_>>());
    }

    fn test_sort(items: &mut [i64], expected: &[i64]) {
        bubble_sort(items);
        assert_eq!(items, expected);
    }
}
