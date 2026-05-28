//! Run this file with `cargo test --test 05_split_items`.

//! TODO: Implement a struct called `SplitItems`, which will receive a string slice and a delimiter
//! char in its constructor.
//!
//! The struct should act as an iterator which iterates over all substrings of the input, separated
//! by the delimiter. The iterator should never return an empty string; it should automatically skip
//! over empty strings.
//!
//! The iterator has to be **lazy**! It should not copy the whole input array
//! (in other words, it should have space complexity O(1)).

struct SplitItems<'a> {
    value: &'a str,
    delimiter: char,
    cursor: usize,
}

impl<'a> SplitItems<'a> {
    fn new(value: &'a str, delimiter: char) -> Self {
        Self {
            value,
            delimiter,
            cursor: 0,
        }
    }
}

impl<'a> Iterator for SplitItems<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value.is_empty() {
            return None;
        }

        let remaining = &self.value[self.cursor..];
        let mut remaining_char_indices = remaining.char_indices();
        let mut remaining_cursor = 0;
        // It saves the first char index of the substring. We consider substring characters all
        // chars that are not delimiters
        let mut start_substring_index: Option<usize> = None;

        loop {
            match remaining_char_indices.next() {
                // We still have chars to traverse from the string
                Some((i, c)) => {
                    self.cursor += 1;
                    remaining_cursor += 1;

                    // When we already found the substring, we continue getting characters until we
                    // find a delimiter. Then, we just return the substring by using the
                    // start_substring_index and the remaining_cursor position.
                    match start_substring_index {
                        Some(start_index) => {
                            if c != self.delimiter {
                                continue;
                            }

                            return Some(&remaining[start_index..remaining_cursor - 1]);
                        }
                        // When we still did not find any character different than a delimiter,
                        // then we just continue traversing the string until we find one. Whenever
                        // that happens, we use that index as the start of the substring.
                        None => {
                            if c == self.delimiter {
                                continue;
                            }

                            start_substring_index = Some(i);
                        }
                    };
                }
                // We already traverse all chars from the string
                None => match start_substring_index {
                    Some(start_index) => return Some(&remaining[start_index..]),
                    None => return None,
                },
            }
        }
    }
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::SplitItems;

    #[test]
    fn split_empty() {
        let result = SplitItems::new("", ' ').collect::<Vec<_>>();
        assert!(result.is_empty());
    }

    #[test]
    fn split_one_delimiter() {
        let result = SplitItems::new("c", 'c').collect::<Vec<_>>();
        assert!(result.is_empty());
    }

    #[test]
    fn split_only_delimiters() {
        let result = SplitItems::new("ccc", 'c').collect::<Vec<_>>();
        assert!(result.is_empty());
    }

    #[test]
    fn split_delimiter_at_beginning() {
        let result = SplitItems::new(" asd", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["asd"]);
    }

    #[test]
    fn split_delimiters_at_beginning() {
        let result = SplitItems::new("  asd", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["asd"]);
    }

    #[test]
    fn split_delimiter_at_end() {
        let result = SplitItems::new("asd ", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["asd"]);
    }

    #[test]
    fn split_delimiters_at_end() {
        let result = SplitItems::new("asd  ", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["asd"]);
    }

    #[test]
    fn split_single_chars() {
        let result = SplitItems::new("a b c d e", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn split_complex() {
        let result = SplitItems::new("   abc   bde casdqw dee xe ", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["abc", "bde", "casdqw", "dee", "xe"]);
    }

    #[test]
    fn split_complex_custom_delimiter() {
        let result = SplitItems::new("pppabcpppbdepcasdqwpdeepxep", 'p').collect::<Vec<_>>();
        assert_eq!(result, vec!["abc", "bde", "casdqw", "dee", "xe"]);
    }

    #[test]
    fn split_check_reference() {
        let data = String::from("foo bar");
        let result = SplitItems::new(&data, ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["foo", "bar"]);
    }

    #[test]
    fn split_check_type() {
        let result: SplitItems<'_> = SplitItems::new("foo bar baz", ' ');
        assert_eq!(result.collect::<Vec<_>>(), vec!["foo", "bar", "baz"]);
    }
}
