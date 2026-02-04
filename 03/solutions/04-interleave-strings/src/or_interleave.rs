//! Run this file with `cargo test --test 03_interleave`.

// TODO: Implement a function called `interleave`, which will take two string slices and return
// a string that contains the characters from the two input strings interleaved.
// The first character of the output should start with the first character of the first argument.
// See tests for details.
//
// Can you write the function without any explicit indexing (`str[index]`)?
//
// Hint: you can use `string.chars()` to create an iterator over the Unicode characters of a string.

pub fn interleave(str1: &str, str2: &str) -> String {
    let mut i_str1 = str1.chars();
    let mut i_str2 = str2.chars();
    let mut result = String::new();
    let mut eo_str1 = false;
    let mut eo_str2 = false;
    loop {
        if !eo_str1 {
            match i_str1.next() {
                Some(c) => result.push(c),
                None => eo_str1 = true,
            };
        }
        if !eo_str2 {
            match i_str2.next() {
                Some(c) => result.push(c),
                None => eo_str2 = true,
            };
        }
        if eo_str1 && eo_str2 {
            break;
        }
    }
    result
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
