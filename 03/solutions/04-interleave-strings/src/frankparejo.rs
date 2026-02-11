//! Run this file with `cargo test --test 03_interleave`

// TODO: Implement a function called `interleave`, which will take two string slices and return
// a string that contains the characters from the two input strings interleaved.
// The first character of the output should start with the first character of the first argument.
// See tests for details.
//
// Can you write the function without any explicit indexing (`str[index]`)?
//
// Hint: you can use `string.chars()` to create an iterator over the Unicode characters of a string.

fn interleave(s1: &str, s2: &str) -> String {
    let s1_len = s1.chars().count();
    let s2_len = s2.chars().count();
    let result = String::with_capacity(s1_len + s2_len);

    // Zipping the s1 string works fine when s1 and s2 have the same length. When they have
    // different, the result is not what we are expecting for this particular solution. The
    // reason is because .zip method stops as soon as one of the iterators ends, which means that
    // the longest string will be shrinked. That's why we need the to chain the missing chars.
    s1.chars()
        .zip(s2.chars())
        .fold(result, |mut acc, (c1, c2)| {
            acc.push(c1);
            acc.push(c2);

            acc
        })
        .chars()
        .chain(if s2_len > s1_len {
            s2.chars().skip(s1_len)
        } else {
            s1.chars().skip(s2_len)
        })
        .collect()
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::interleave;

    #[test]
    fn interleave_empty() {
        assert_eq!(interleave("", ""), "");
    }

    #[test]
    fn interleave_only_left() {
        assert_eq!(interleave("a", ""), "a");
        assert_eq!(interleave("zxjas", ""), "zxjas");
    }

    #[test]
    fn interleave_only_right() {
        assert_eq!(interleave("", "z"), "z");
        assert_eq!(interleave("", "foobar"), "foobar");
    }

    #[test]
    fn interleave_same_length() {
        assert_eq!(interleave("abcdef", "012345"), "a0b1c2d3e4f5");
    }

    #[test]
    fn interleave_first_longer() {
        assert_eq!(
            interleave("Programming Rust", "O'Reilly"),
            "POr'oRgerialmlmying Rust"
        );
    }

    #[test]
    fn interleave_second_longer() {
        assert_eq!(
            interleave("ahoj, jak se máš?", "díky za optání, mám se dobře"),
            "adhíokjy,  zjaa ko psteá nmíá,š ?mám se dobře"
        );
    }
}
