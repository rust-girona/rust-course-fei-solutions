//! Run this file with `cargo test --test 02_case_insensitive_cmp`.

//! TODO: Implement a struct `CaseInsensitive`, which will allow comparing (=, <, >, etc.)
//! two (ASCII) string slices in a case insensitive way, without performing any reallocations
//! and without modifying the original strings.

use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

#[derive(Eq)]
struct CaseInsensitive<'a>(&'a str);

impl<'a> PartialEq for CaseInsensitive<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(other.0)
    }
}

impl<'a> PartialOrd for CaseInsensitive<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut other_iter = other.0.chars();

        for self_c in self.0.chars() {
            if let Some(self_o) = other_iter.next() {
                // TODO make the comparison with eq_ignore_ascii_case
            } else {
                return Some(Ordering::Greater);
            }
        }

        Some(Ordering::Equal)
    }
}

impl<'a> Ord for CaseInsensitive<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
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
}
