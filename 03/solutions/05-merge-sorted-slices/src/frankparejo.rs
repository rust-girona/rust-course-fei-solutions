//! Run this file with `cargo test --test 04_merge_slices`.

//! TODO: Implement a function called `merge_slices`, which is useful for the merge sort algorithm.
//! It will take two sorted `u32` slices as inputs and merge them into a sorted vector (Vec).
//! The function will return the vector.
//!
//! The time complexity of this function has to be O(n).
//!
//! Bonus: Can you build a complete merge sort on top of this function? :)

fn merge_slices(s1: &[u32], s2: &[u32]) -> Vec<u32> {
    let mut merged_slices: Vec<u32> = Vec::with_capacity(s1.len() + s2.len());
    let mut s1_iter = s1.iter().peekable();
    let mut s2_iter = s2.iter().peekable();

    loop {
        // Gets the number from each array without moving the cursor (that's why we called peek
        // instead of next. It will be moved only when the number is included into the
        // merged_slices by calling the next function.
        let n1 = s1_iter.peek().cloned();
        let n2 = s2_iter.peek().cloned();

        match (n1, n2) {
            (Some(v), None) => {
                merged_slices.push(*v);

                s1_iter.next();
            }
            (None, Some(v)) => {
                merged_slices.push(*v);

                s2_iter.next();
            }
            (Some(v1), Some(v2)) => {
                if v1 < v2 {
                    merged_slices.push(*v1);

                    s1_iter.next();
                } else {
                    merged_slices.push(*v2);

                    s2_iter.next();
                }
            }
            (None, None) => break,
        }
    }

    merged_slices
}

fn merge_sort(s: &[u32]) -> Vec<u32> {
    let len = s.len();

    if len <= 1 {
        Vec::from(s)
    } else {
        let half = len.div_ceil(2);

        merge_slices(&merge_sort(&s[0..half]), &merge_sort(&s[half..]))
    }
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::{merge_slices, merge_sort};

    #[test]
    fn merge_slices_empty() {
        assert_eq!(merge_slices(&[], &[]), vec![]);
    }

    #[test]
    fn merge_slices_basic() {
        assert_eq!(merge_slices(&[1, 2, 3], &[4, 5, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn merge_slices_interleaved() {
        assert_eq!(merge_slices(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn merge_slices_duplicates() {
        assert_eq!(merge_slices(&[1, 1, 3], &[1, 3, 4]), vec![1, 1, 1, 3, 3, 4]);
    }

    #[test]
    fn merge_slices_uneven_size() {
        assert_eq!(
            merge_slices(&[1, 4, 6, 8], &[0, 1, 1, 3, 4, 5, 7, 8, 9]),
            vec![0, 1, 1, 1, 3, 4, 4, 5, 6, 7, 8, 8, 9]
        );
    }

    #[test]
    fn merge_slices_first_empty() {
        assert_eq!(merge_slices(&[], &[1, 4, 8]), vec![1, 4, 8]);
    }

    #[test]
    fn merge_slices_second_empty() {
        assert_eq!(merge_slices(&[1, 9, 11], &[]), vec![1, 9, 11]);
    }

    #[test]
    fn merge_sort_basic() {
        assert_eq!(merge_sort(&[1, 3, 5, 2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn merge_sort_duplicates() {
        assert_eq!(merge_sort(&[1, 1, 3, 1, 3, 4]), vec![1, 1, 1, 3, 3, 4]);
    }
}
