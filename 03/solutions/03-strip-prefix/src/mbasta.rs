//! Run this file with `cargo test --test 02_strip_prefix`.

// TODO: Implement a function called `strip_prefix`, which will take two strings (`needle` and `prefix`).
// It will return a substring of `needle` starting at the first character that is not contained
// in `prefix`.
// See tests for examples. Take a look at the `strip_prefix_lifetime_check` test!
//
// Hint: you can use `string.chars()` for iterating the Unicode characters of a string.

use std::collections::HashSet;

fn strip_prefix<'a, 'b>(needle: &'a str, prefix: &'b str) -> &'a str {
    let prefix: HashSet<char> = prefix.chars().collect();
    // needle.chars().enumerate() counts the nuber of iterations, but because each Char can have a
    // different size in bytes. The index of a Char in the string slice doesn't match the iteration count.
    // `char_indices` returns for each Char its correct index in the string slice.
    for (idx, char) in needle.char_indices() {
        if !prefix.contains(&char) {
            return &needle[idx..];
        }
    }
    ""
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::strip_prefix;

    #[test]
    fn strip_prefix_basic() {
        assert_eq!(strip_prefix("foobar", "of"), "bar");
    }

    #[test]
    fn strip_prefix_full_result() {
        assert_eq!(strip_prefix("foobar", "x"), "foobar");
    }

    #[test]
    fn strip_prefix_empty_result() {
        assert_eq!(strip_prefix("foobar", "fbaro"), "");
    }

    #[test]
    fn strip_prefix_unicode() {
        assert_eq!(strip_prefix("čaukymňauky", "čaukym"), "ňauky");
    }

    #[test]
    fn strip_prefix_lifetime_check() {
        let needle = "foobar";
        let prefix = String::from("f");
        let result = strip_prefix(needle, &prefix);
        drop(prefix);
        assert_eq!(result, "oobar");
    }
}
