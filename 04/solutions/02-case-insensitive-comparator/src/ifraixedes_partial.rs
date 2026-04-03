//! Run this file with `cargo test --test 02_case_insensitive_cmp`.

//! TODO: Implement a struct `CaseInsensitive`, which will allow comparing (=, <, >, etc.)
//! two (ASCII) string slices in a case insensitive way, without performing any reallocations
//! and without modifying the original strings.

use std::cmp::{Ordering, PartialEq, PartialOrd};

struct CaseInsensitive<'a>(&'a str);

impl<'a> PartialEq for CaseInsensitive<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(other.0)
    }
}

impl<'a> PartialOrd for CaseInsensitive<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut other_iter = other.0.chars();

        for mut self_c in self.0.chars() {
            if let Some(mut other_c) = other_iter.next() {
                if !self_c.eq_ignore_ascii_case(&other_c) {
                    // NOTE the exercise specifies to compare two ASCII characters, so if one has a
                    // non ASCII character we consider that we cannot order them. This similar to
                    // what happens with floats and the NAN value.
                    if !self_c.is_ascii() || !other_c.is_ascii() {
                        return None;
                    }

                    // NOTE The following instructions don't modify the original string because they
                    // are copies. This is guarantee by the compiler because self and other are
                    // immutable references and if these will mutate them, the compiler will yell
                    // about it.
                    self_c.make_ascii_lowercase();
                    other_c.make_ascii_lowercase();
                    if self_c < other_c {
                        return Some(Ordering::Less);
                    } else {
                        return Some(Ordering::Greater);
                    }
                }
            } else {
                return Some(Ordering::Greater);
            }
        }

        other_iter
            .next()
            .map_or(Some(Ordering::Equal), |_| Some(Ordering::Less))
    }
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::CaseInsensitive;

    #[test]
    fn case_insensitive_same() {
        assert!(CaseInsensitive("") == CaseInsensitive(""));
        assert!(CaseInsensitive("a") == CaseInsensitive("A"));
        assert!(CaseInsensitive("a") == CaseInsensitive("a"));
        assert!(CaseInsensitive("Foo") == CaseInsensitive(&String::from("fOo")));
        assert!(
            CaseInsensitive("12ABBBcLPQusdaweliAS2") == CaseInsensitive("12AbbbclpQUSdawelias2")
        );
    }

    #[test]
    fn case_insensitive_smaller() {
        assert!(CaseInsensitive("") < CaseInsensitive("a"));
        assert!(CaseInsensitive("a") < CaseInsensitive("B"));
        assert!(CaseInsensitive("aZa") < CaseInsensitive("Zac"));
        assert!(CaseInsensitive("aZ") < CaseInsensitive("Zac"));
        assert!(CaseInsensitive("PWEasUDsx") < CaseInsensitive("PWEaszDsx"));
        assert!(CaseInsensitive("PWEasuDsx") < CaseInsensitive("PWEasZDsx"));
    }

    #[test]
    fn case_insensitive_larger() {
        assert!(CaseInsensitive("a") > CaseInsensitive(""));
        assert!(CaseInsensitive("B") > CaseInsensitive("a"));
        assert!(CaseInsensitive("Zac") > CaseInsensitive("aZa"));
        assert!(CaseInsensitive("Zac") > CaseInsensitive("aZ"));
        assert!(CaseInsensitive("PWEaszDsx") > CaseInsensitive("PWEasUDsx"));
        assert!(CaseInsensitive("PWEasZDsx") > CaseInsensitive("PWEasuDsx"));
    }

    #[test]
    fn case_insensitive_less_or_equal() {
        assert!(CaseInsensitive("aZ") <= CaseInsensitive("Zac"));
        assert!(CaseInsensitive("Zac") <= CaseInsensitive("Zac"));
    }

    #[test]
    fn case_insensitive_greater_or_equal() {
        assert!(CaseInsensitive("Zac") >= CaseInsensitive("aZ"));
        assert!(CaseInsensitive("Zac") >= CaseInsensitive("Zac"));
    }

    #[test]
    fn case_non_ascii() {
        // To make these always false the partial_cmp must be implemented and not based in Ord
        // implementation
        assert!(!(CaseInsensitive("a") < CaseInsensitive("❤")));
        assert!(!(CaseInsensitive("a") > CaseInsensitive("❤")));
        assert!(CaseInsensitive("❤") == CaseInsensitive("❤"));
    }
}
