//! Run this file with `cargo test --test 05_bubble_sort`.

fn bubble_sort(items: &mut [i64]) {
    let size = items.len();
    if size <= 1 {
        return;
    }

    for i in 0..size {
        for x in 0..(size - 1 - i) {
            let a = items[x];
            let b = items[x + 1];

            if a > b {
                items[x] = b;
                items[x + 1] = a;
            }
        }
    }
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::magonxesp::bubble_sort;
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
        data.shuffle(&mut rand::thread_rng());

        test_sort(&mut data, &(0..1000).collect::<Vec<_>>());
    }

    fn test_sort(items: &mut [i64], expected: &[i64]) {
        bubble_sort(items);
        assert_eq!(items, expected);
    }
}
