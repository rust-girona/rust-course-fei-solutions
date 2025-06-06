fn find_largest(array: [i32; 10]) -> i32 {
    // You can replace this conditional with `if array.len() == 0` but then Clippy will complain
    // with `len_zero` https://rust-lang.github.io/rust-clippy/master/index.html#len_zero
    if array.is_empty() {
        return i32::MIN;
    }

    let mut i = 0;
    let mut max: i32 = array[0];
    loop {
        i += 1;

        if i >= array.len() {
            break;
        }

        if array[i] > max {
            max = array[i];
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::find_largest;

    #[test]
    fn find_largest_all_same() {
        assert_eq!(find_largest([2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), 2);
    }

    #[test]
    fn find_largest_increasing() {
        assert_eq!(find_largest([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 10);
    }

    #[test]
    fn find_largest_decreasing() {
        assert_eq!(find_largest([10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 10);
    }

    #[test]
    fn find_largest_random() {
        assert_eq!(find_largest([17, 10, 18, 3, 7, 8, 7, 19, 20, 8]), 20);
    }

    #[test]
    fn find_largest_negative() {
        assert_eq!(
            find_largest([-17, -10, -18, -3, -7, -8, -7, -19, -20, -8]),
            -3
        );
    }

    #[test]
    fn find_maximum_i32() {
        assert_eq!(
            find_largest([17, 10, 18, 3, 7, i32::MAX, 7, 19, 20, 8]),
            i32::MAX,
        );
    }
}
