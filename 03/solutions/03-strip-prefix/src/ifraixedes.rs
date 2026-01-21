//! Run this file with `cargo test --test 02_strip_prefix`.

// TODO: Implement a function called `strip_prefix`, which will take two strings (`needle` and `prefix`).
// It will return a substring of `needle` starting at the first character that is not contained
// in `prefix`.
// See tests for examples. Take a look at the `strip_prefix_lifetime_check` test!
//
// Hint: you can use `string.chars()` for iterating the Unicode characters of a string.

// My initial implementation had this function signature:
// `fn strip_prefix<'a, 'b>(needle: &'a str, prefix: &'b str) -> &'a str {`
// Clippy gave this warning:
// The following explicit lifetimes could be elided: 'b
// fn strip_prefix<'a, 'b>(needle: &'a str, prefix: &'b str) -> &'a str {
//
// Which means that the compiler can infers its lifetime independently .
//
// ## What lifetime does prefix have?
// It has its own unique anonymous lifetime, determined by the scope in which the argument is valid.
// This lifetime is independent of 'a — they are not related.
// The compiler treats this as if you wrote:
// fn strip_prefix<'a, 'b>(needle: &'a str, prefix: &'b str) -> &'a str
//
// but since 'b is only used once and doesn't affect the return type, it can be elided.
//
// ## Key Point:
// Even though 'b isn't written, it still exists internally. The prefix reference must live long
// enough for the function call itself, but its lifetime has no relationship to the returned string
// slice (which is borrowed from needle and thus tied to 'a).
//
// This is safe because:
//
// - The return value is derived from needle, not prefix.
// - prefix is only used during the call and doesn't need to outlive 'a.
//   So: prefix has an independent, elided lifetime — inferred by Rust and valid only for the
//   duration of the function call.

fn strip_prefix<'a>(needle: &'a str, prefix: &str) -> &'a str {
    for (i, c) in needle.char_indices() {
        if !prefix.contains(c) {
            return &needle[i..];
        }
    }

    ""
}

// The NOTE below was WRONG and I should have done what Oscar Rambla did (look at or_strip.rs).
//
// NOTE despite of the "Hint" indicated in the exercise about using `string.chars()`, I couldn't
// find a way to make the unicode test to pass, which should be correct because the documentation
// of the method says:
//
// > It’s important to remember that char represents a Unicode Scalar Value, and might not match
// > your idea of what a ‘character’ is. Iteration over grapheme clusters may be what you actually
// > want. This functionality is not provided by Rust’s standard library, check crates.io instead.
//
// And it gives an example with an string that contains unicode character and how some characters
// aren't what we may expect.
//
// I tried anyway with the 2 implementations below:
/*
fn strip_prefix<'a, 'b>(needle: &'a str, prefix: &'b str) -> &'a str {
    let mut num_matches = 0;
    for c in needle.chars() {
        if !prefix.contains(c) {
            break;
        }
        num_matches += 1;
    }

    &needle[num_matches..]
}

fn strip_prefix<'a, 'b>(needle: &'a str, prefix: &'b str) -> &'a str {
    let mut num_matches = 0;
    for (i, c) in needle.chars().enumerate() {
        if !prefix.contains(c) {
            return &needle[i..];
        }
        num_matches += 1;
    }

    &needle[num_matches..]
}
*/

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
    fn strip_prefix_unicode_full_match() {
        assert_eq!(strip_prefix("čaukymauky", "čaukym"), "");
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
