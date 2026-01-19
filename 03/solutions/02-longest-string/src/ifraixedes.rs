// TODO: Implement a function called `longest`, which will return the longer of the two
// input strings. If they are the same length, return the first string.

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::longest;

    #[test]
    fn longest_first_longer() {
        let result = longest("aqwe", "ab");
        assert_eq!(result, "aqwe");
    }

    #[test]
    fn longest_second_longer() {
        let result = longest("foo", "barbaz");
        assert_eq!(result, "barbaz");
    }

    #[test]
    fn longest_same_length() {
        let result = longest("x", "y");
        assert_eq!(result, "x");
    }

    #[test]
    fn longest_different_lifetimes_unified() {
        let a = "foo";
        let b = String::from("barx");
        let result = longest(a, &b);
        assert_eq!(result, &b);
    }

    // TODO: Can we write the `longest` function in a way that the following test compiles?
    // The function has to return one of the two input strings, and cannot copy the string data.
    // `longest` should return the first string in this case, so it should be fine to
    // drop `b`. Right? :)
    //
    // ANSWER: No, it is not possible in safe Rust. The compiler checks lifetimes at compile time.
    // The signature `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str` implies the result
    // lives as long as the shorter of x and y, which we cannot determine at compile time because
    // the function return one or another according to their length. Because both must have the same
    // lifetime and b is dropped right after the call and before the result and this has to the
    // shortest arguments lifetime, it doesn't compile.
    /*
    #[test]
    fn longest_different_lifetimes_drop() {
        let a = "longer";
        let b = String::from("short");
        let result = longest(a, &b);
        drop(b);
        assert_eq!(result, "longer");
    }
    */
}
