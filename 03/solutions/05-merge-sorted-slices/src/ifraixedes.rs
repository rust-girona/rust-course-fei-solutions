//! TODO: Implement a function called `merge_slices`, which is useful for the merge sort algorithm.
//! It will take two sorted `u32` slices as inputs and merge them into a sorted vector (Vec).
//! The function will return the vector.
//!
//! The time complexity of this function has to be O(n).
//!
//! Bonus: Can you build a complete merge sort on top of this function? :)
//! Answer: Yes, because merge sort is a divide and conquer algorithm that divides unsorted list
//! into n sub-lists each containing one element, those are considered sorted lists and then
//! repeatedly merged sublists to produce new sorted sublists until there is only one sublist
//! remaining, which will be the sorted list. This is implemented below in the function `merge_sort`
//! with some tests.

fn merge_slices(s1: &[u32], s2: &[u32]) -> Vec<u32> {
    let mut merged = Vec::with_capacity(s1.len() + s2.len());

    let mut it1 = s1.iter();
    let mut it2 = s2.iter();
    let mut previous = (None, None);
    loop {
        let (opt1, opt2) = match previous {
            (Some(u1), None) => (Some(u1), it2.next()),
            (None, Some(u2)) => (it1.next(), Some(u2)),
            (None, None) => (it1.next(), it2.next()),
            (Some(_), Some(_)) => {
                panic!("BUG: previous call left to handle both elements for the next iteration")
            }
        };

        previous = match (opt1, opt2) {
            (Some(u1), None) => {
                merged.push(*u1);
                // NOTE: instead of calling extend with the rest of the items of `it1` and break the
                // loop we could assign `(None, None)` to previous and continue iterating.
                // Using extend we may get some performance because we avoid the pattern matching
                // for the rest of the elements of `it1` and perhaps the compiler can optimize the
                // assembly to add those elements to `merged` vector.
                merged.extend(&mut it1);
                break;
            }
            (None, Some(u2)) => {
                merged.push(*u2);
                // NOTE: instead of calling extend with the rest of the items of `it2` and break the
                // loop we could assign `(None, None)` to previous and continue iterating.
                // Using extend we may get some performance because we avoid the pattern matching
                // for the rest of the elements of `it2` and perhaps the compiler can optimize the
                // assembly to add those elements to `merged` vector.
                merged.extend(&mut it2);
                break;
            }
            (Some(u1), Some(u2)) => {
                if u1 <= u2 {
                    merged.push(*u1);
                    (None, Some(u2))
                } else {
                    merged.push(*u2);
                    (Some(u1), None)
                }
            }
            (None, None) => break,
        };
    }

    merged
}

fn merge_sort(list: &[u32]) -> Vec<u32> {
    if list.len() <= 1 {
        return Vec::from(list);
    }

    let mut splitted = list.chunks(list.len().div_ceil(2));
    let l1 = merge_sort(
        splitted
            .next()
            .expect("BUG: this should have always 2 elements"),
    );
    let l2 = merge_sort(
        splitted
            .next()
            .expect("BUG: this should have always 2 elements"),
    );

    merge_slices(&l1, &l2)
}

fn merge_sort_iterative(list: &[u32]) -> Vec<u32> {
    let mut list = Vec::from(list);
    let mut subsorted = Vec::with_capacity(list.len());
    let mut step = 2;
    while step / 2 < list.len() {
        for i in (0..list.len()).step_by(step) {
            let (lower_bound, upper_bound) = if (step / 2 + i) < list.len() {
                if (i + step) < list.len() {
                    (step / 2 + i, i + step)
                } else {
                    (step / 2 + i, list.len())
                }
            } else {
                (list.len(), list.len())
            };
            let sorted = merge_slices(&list[i..lower_bound], &list[lower_bound..upper_bound]);
            subsorted.extend_from_slice(&sorted);
        }

        // We can do 2 things, one require to allocate a new vector for each iteration of the while
        // loop and the other doesn't require allocation, but it requires to iterate the vector for
        // each iteration of the while loop
        // 1)
        list = subsorted;
        subsorted = Vec::with_capacity(list.len());

        // 2)
        // for (i, v) in subsorted.iter().enumerate() {
        //     list[i] = *v;
        // }
        // subsorted.clear();

        step *= 2;
    }

    list
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests_merge_slices {
    use super::merge_slices;

    #[test]
    fn empty() {
        assert_eq!(merge_slices(&[], &[]), vec![]);
    }

    #[test]
    fn basic() {
        assert_eq!(merge_slices(&[1, 2, 3], &[4, 5, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn interleaved() {
        assert_eq!(merge_slices(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn duplicates() {
        assert_eq!(merge_slices(&[1, 1, 3], &[1, 3, 4]), vec![1, 1, 1, 3, 3, 4]);
    }

    #[test]
    fn uneven_size() {
        assert_eq!(
            merge_slices(&[1, 4, 6, 8], &[0, 1, 1, 3, 4, 5, 7, 8, 9]),
            vec![0, 1, 1, 1, 3, 4, 4, 5, 6, 7, 8, 8, 9]
        );
    }

    #[test]
    fn first_empty() {
        assert_eq!(merge_slices(&[], &[1, 4, 8]), vec![1, 4, 8]);
    }

    #[test]
    fn second_empty() {
        assert_eq!(merge_slices(&[1, 9, 11], &[]), vec![1, 9, 11]);
    }
}

#[cfg(test)]
mod tests_merge_sort {
    use super::merge_sort;

    #[test]
    fn empty() {
        assert_eq!(merge_sort(&[]), Vec::new(),);
    }

    #[test]
    fn one_element() {
        assert_eq!(merge_sort(&[5]), vec![5]);
    }

    #[test]
    fn two_element_sorted() {
        assert_eq!(merge_sort(&[2, 6]), vec![2, 6]);
    }

    #[test]
    fn two_element_unsorted() {
        assert_eq!(merge_sort(&[5, 1]), vec![1, 5]);
    }

    #[test]
    fn long_sorted() {
        assert_eq!(
            merge_sort(&[0, 1, 1, 1, 3, 4, 4, 5, 6, 7, 8, 8, 9]),
            vec![0, 1, 1, 1, 3, 4, 4, 5, 6, 7, 8, 8, 9]
        );
    }

    #[test]
    fn long_unsorted() {
        assert_eq!(
            merge_sort(&[8, 0, 1, 3, 1, 1, 4, 5, 6, 7, 4, 9, 8]),
            vec![0, 1, 1, 1, 3, 4, 4, 5, 6, 7, 8, 8, 9]
        );
    }
}

#[cfg(test)]
mod tests_merge_sort_iterative {
    use super::merge_sort_iterative;

    #[test]
    fn empty() {
        assert_eq!(merge_sort_iterative(&[]), Vec::new(),);
    }

    #[test]
    fn one_element() {
        assert_eq!(merge_sort_iterative(&[5]), vec![5]);
    }

    #[test]
    fn two_element_sorted() {
        assert_eq!(merge_sort_iterative(&[2, 6]), vec![2, 6]);
    }

    #[test]
    fn two_element_unsorted() {
        assert_eq!(merge_sort_iterative(&[5, 1]), vec![1, 5]);
    }

    #[test]
    fn three_element_unsorted() {
        assert_eq!(merge_sort_iterative(&[3, 2, 1]), vec![1, 2, 3]);
    }

    #[test]
    fn five_element_unseroted() {
        assert_eq!(merge_sort_iterative(&[3, 2, 1, 0, 5]), vec![0, 1, 2, 3, 5]);
    }

    #[test]
    fn seven_element_unsorted() {
        assert_eq!(
            merge_sort_iterative(&[3, 2, 1, 5, 7, 0, 8]),
            vec![0, 1, 2, 3, 5, 7, 8]
        );
    }

    #[test]
    fn long_sorted() {
        assert_eq!(
            merge_sort_iterative(&[0, 1, 1, 1, 3, 4, 4, 5, 6, 7, 8, 8, 9]),
            vec![0, 1, 1, 1, 3, 4, 4, 5, 6, 7, 8, 8, 9]
        );
    }

    #[test]
    fn long_unsorted() {
        assert_eq!(
            merge_sort_iterative(&[8, 0, 1, 3, 1, 1, 4, 5, 6, 7, 4, 9, 8]),
            vec![0, 1, 1, 1, 3, 4, 4, 5, 6, 7, 8, 8, 9]
        );
    }
}
