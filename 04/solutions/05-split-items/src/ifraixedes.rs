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
    text: &'a str,
    delimiter: char,
    index_start: usize,
}

impl<'a> SplitItems<'a> {
    fn new(text: &'a str, delimiter: char) -> Self {
        Self {
            text,
            delimiter,
            index_start: 0,
        }
    }
}

impl<'a> Iterator for SplitItems<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut index_end = self.index_start;

        for c in self.text.get(self.index_start..)?.chars() {
            if c == self.delimiter {
                if self.index_start == index_end {
                    self.index_start += 1;
                    index_end = self.index_start;
                    continue;
                }

                let sub = self.text.get(self.index_start..index_end);
                self.index_start = index_end + 1;
                return sub;
            }

            index_end += 1;
        }

        let last = if self.index_start == index_end {
            None
        } else {
            self.text.get(self.index_start..index_end)
        };

        // It has to be `text.len() + 1` because `text.get(text.len()..text.len())` returns "", but
        // it returns none if the initial index is greater than length. The byte at length position
        // is boundary after the last byte which lies on UTF-8 code point boundaries.
        // This guarantees that next iteration will return None and not enter in an infinite loop.
        self.index_start = index_end + 1;
        last
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
