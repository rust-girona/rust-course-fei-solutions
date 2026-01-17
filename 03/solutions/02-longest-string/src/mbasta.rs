fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if second.chars().count() > first.chars().count() {
        second
    } else {
        first
    }
}

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

    #[test]
    fn longest_first_longer_second_larger() {
        let result = longest("foox", "föö");
        assert_eq!(result, "foox");
    }

    // Can we write the `longest` function in a way that the following test compiles?
    // The function has to return one of the two input strings, and cannot copy the string data.
    // `longest` should return the first string in this case, so it should be fine to
    // drop `b`. Right? :)
    // This is generally not possible because `longest` would need to know at compile-time which
    // value it would return. That would only be possible for static strings.
    // #[test]
    // fn longest_different_lifetimes_drop() {
    //     let a = "longer";
    //     let b = String::from("short");
    //     let result = longest(a, &b);
    //     drop(b);
    //     assert_eq!(result, "longer");
    // }
}
