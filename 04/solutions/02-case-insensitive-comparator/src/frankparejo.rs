//! Run this file with `cargo test --test 02_case_insensitive_cmp`.

//! TODO: Implement a struct `CaseInsensitive`, which will allow comparing (=, <, >, etc.)
//! two (ASCII) string slices in a case insensitive way, without performing any reallocations
//! and without modifying the original strings.

use std::cmp::{self, Ordering};

struct CaseInsensitive<'a>(&'a str);

impl PartialEq for CaseInsensitive<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(other.0)
    }
}

impl PartialOrd for CaseInsensitive<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut s1 = self.0.chars();
        let mut s2 = other.0.chars();
        let mut ordering: Option<Ordering> = None;

        loop {
            let c1 = s1.next();
            let c2 = s2.next();

            match (c1, c2) {
                (None, Some(_)) => {
                    ordering = Some(Ordering::Less);
                    break;
                }
                (Some(_), None) => {
                    ordering = Some(Ordering::Greater);
                    break;
                }
                (Some(c1), Some(c2)) => {
                    let c1 = if c1.is_ascii_lowercase() {
                        c1 as u8
                    } else {
                        (c1 as u8).to_ascii_lowercase()
                    };
                    let c2 = if c2.is_ascii_lowercase() {
                        c2 as u8
                    } else {
                        (c2 as u8).to_ascii_lowercase()
                    };

                    if c1 > c2 {
                        ordering = Some(Ordering::Greater);
                        break;
                    }

                    if c1 < c2 {
                        ordering = Some(Ordering::Less);
                        break;
                    }

                    ordering = Some(Ordering::Equal)
                }
                (None, None) => {
                    break;
                }
            };
        }

        ordering
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
}
