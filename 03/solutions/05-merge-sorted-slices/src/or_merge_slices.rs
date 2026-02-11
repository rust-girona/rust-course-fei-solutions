//! Run this file with `cargo test --test 04_merge_slices`.

//! TODO: Implement a function called `merge_slices`, which is useful for the merge sort algorithm.
//! It will take two sorted `u32` slices as inputs and merge them into a sorted vector (Vec).
//! The function will return the vector.
//!
//! The time complexity of this function has to be O(n).
//!
//! Bonus: Can you build a complete merge sort on top of this function? :)
//!
pub fn merge_slices(slice1: &[u32], slice2: &[u32]) -> Vec<u32> {
    let mut result = Vec::with_capacity(slice1.len() + slice2.len());
    let mut i = 0;
    let mut j = 0;

    while i < slice1.len() && j < slice2.len() {
        if slice1[i] < slice2[j] {
            result.push(slice1[i]);
            i += 1;
        } else {
            result.push(slice2[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&slice1[i..]);
    result.extend_from_slice(&slice2[j..]);

    result
}

pub fn merge_sort(vec: Vec<u32>) -> Vec<u32> {
    if vec.len() <= 1 {
        return vec.clone();
    }

    let mid = vec.len() / 2;
    let mut left = merge_sort(vec[..mid].to_vec());
    let mut right = merge_sort(vec[mid..].to_vec());

    merge_slices(&left, &right)
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::merge_slices;
    use super::merge_sort;

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
        assert_eq!(merge_slices(&[11, 9, 10], &[]), vec![11, 9, 10]);
    }

    #[test]
    fn test_merge_sort() {
        assert_eq!(merge_sort(vec![11, 9, 10]), vec![9, 10, 11]);
    }
}
