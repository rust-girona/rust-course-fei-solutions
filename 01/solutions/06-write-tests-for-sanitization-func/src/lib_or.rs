//! Run this file with `cargo test --test 06_write_tests`.

/// This function implements a string sanitization logic that should uphold the following
/// properties:
/// - After sanitization, the result must not end with the character `x`
/// - After sanitization, the result must not end with the character `o`
/// - After sanitization, the result must not end with the string `.exe`
///
/// The function assumes that the input to the function only consists of lower and uppercase
/// characters from the English alphabet and digits 0-9.
///
/// The implementation contains some bugs.
///
/// Your task is to write a set (at least 8) of unit tests, use them to find (at least 2) bugs in
/// this function and then fix the function.

fn sanitize(input: &str) -> &str {
    let mut mark = true;
    let mut input1 = input;

    while mark && input1.len()>0 {
        if input1.ends_with("x"){
            mark = true;
            input1 = input1.trim_end_matches("x");
        }
        else
        if input1.ends_with("o"){
            mark = true;
            input1 = input1.trim_end_matches("o");
        }
        else
        if input1.ends_with(".exe"){
            mark = true;
            input1 = input1.trim_end_matches(".exe");
        }
        else{
            mark = false;
        }
    }
    input1
}

#[cfg(test)]
mod pepe {
    use super::sanitize;

    #[test]
    fn test_empty() {
        assert_eq!(sanitize(""), "");
    }

    #[test]
    fn test_correct() {
        assert_eq!(sanitize("123"), "123");
    }

    #[test]
    fn test_remove_x() {
        assert_eq!(sanitize("ix"), "i");
        assert_eq!(sanitize("ox"), "");
    }

    #[test]
    fn test_fail_x() {
        assert_eq!(sanitize("xox"), "");
    }

    #[test]
    fn test_exe() {
        assert_eq!(sanitize(".exeo.exe"), "");
    }

    #[test]
    fn test_exe1() {
        assert_eq!(sanitize("o.exex"), "");
    }

    #[test]
    fn test_exe2() {
        assert_eq!(sanitize(".exei.exe"), ".exei");
    }

}
