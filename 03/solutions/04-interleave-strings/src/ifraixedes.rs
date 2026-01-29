// TODO: Implement a function called `interleave`, which will take two string slices and return
// a string that contains the characters from the two input strings interleaved.
// The first character of the output should start with the first character of the first argument.
// See tests for details.
//
// Can you write the function without any explicit indexing (`str[index]`)?
//
// Hint: you can use `string.chars()` to create an iterator over the Unicode characters of a string.

use std::borrow::Cow;

fn interleave<'a>(s1: &'a str, s2: &'a str) -> Cow<'a, str> {
    if s1.is_empty() {
        return Cow::Borrowed(s2);
    }

    if s2.is_empty() {
        return Cow::Borrowed(s1);
    }

    // Allocate the space in the heap in advance, to avoid the multiple allocations penalty.
    let mut interleaved = String::with_capacity(s1.len() + s2.len());
    let mut chars1 = s1.chars();
    let mut chars2 = s2.chars();
    loop {
        if let Some(c1) = chars1.next() {
            interleaved.push(c1);
        } else {
            interleaved.push_str(chars2.as_str());
            break;
        }

        if let Some(c2) = chars2.next() {
            interleaved.push(c2);
        } else {
            interleaved.push_str(chars1.as_str());
            break;
        }
    }

    Cow::Owned(interleaved)
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
