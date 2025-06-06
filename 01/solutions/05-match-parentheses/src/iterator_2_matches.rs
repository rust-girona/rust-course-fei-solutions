fn match_parentheses(s: &str) -> bool {
    use std::vec::Vec;
    let mut stack: Vec<char> = Vec::with_capacity(if s.len() <= 100 { s.len() } else { 100 });

    let verification = s.chars().enumerate().try_for_each(|(i, c)| match c {
        '(' | '[' | '{' => {
            stack.push(c);
            Ok(())
        }
        ')' | ']' | '}' => {
            let correct = match stack.pop() {
                Some('(') => c == ')',
                Some('[') => c == ']',
                Some('{') => c == '}',
                Some(_) => {
                    panic!("BUG: stack cannot contain any other character than '(', '[', '{{'")
                }
                None => false,
            };

            if correct {
                Ok(())
            } else {
                // TODO: position doesn't match if the string contains unicode characters.
                Err(format!("Unpaired opener for '{}' at position {}", c, i))
            }
        }
        _ => Ok(()),
    });

    verification.is_ok() && stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::match_parentheses;

    #[test]
    fn match_parentheses_empty() {
        assert!(match_parentheses(""));
        assert!(match_parentheses("foobar"));
    }

    #[test]
    fn match_parentheses_wrong_order() {
        assert!(!match_parentheses(")("));
        assert!(!match_parentheses("xx)y(aa"));
    }

    #[test]
    fn match_parentheses_extra_opening() {
        assert!(!match_parentheses("("));
        assert!(!match_parentheses("x[qq]e(s"));
        assert!(!match_parentheses("[(]"));
        assert!(!match_parentheses("(xxú[ú]😊"));
    }

    #[test]
    fn match_parentheses_extra_closing() {
        assert!(!match_parentheses(")"));
        assert!(!match_parentheses("[])"));
        assert!(!match_parentheses("[)]"));
        assert!(!match_parentheses("x([{)}])y"));
    }

    #[test]
    fn match_parentheses_wrong_matched_type() {
        assert!(!match_parentheses("[)"));
        assert!(!match_parentheses("[qp)"));
        assert!(!match_parentheses("[}xx"));
        assert!(!match_parentheses("p{]vr"));
        assert!(!match_parentheses("y[q}xx"));
        assert!(!match_parentheses("y[qq)x"));
        assert!(!match_parentheses("([})"));
        assert!(!match_parentheses("(((([}))))"));
    }

    #[test]
    fn respect_relative_ordering() {
        assert!(!match_parentheses("([)]"));
    }

    #[test]
    fn simple() {
        assert!(match_parentheses("[]"));
        assert!(match_parentheses("()"));
        assert!(match_parentheses("{}"));
        assert!(match_parentheses("{}{}"));
        assert!(match_parentheses("{}{}{}"));
        assert!(match_parentheses("{}[]()"));
        assert!(match_parentheses("y{x}qq(xywe)[][xx]yy[][p]()"));
    }

    #[test]
    fn nested() {
        assert!(match_parentheses("({[]})"));
        assert!(match_parentheses("q(x{x[xqp]yy}y)"));
        assert!(match_parentheses("((((()))))"));
        assert!(match_parentheses("{[({})](([]))}"));
    }
}
