//! Run this file with `cargo test --test 05_srl_validator`.

// TODO: Implement a SRL (Simple Resource Locator) validator.
// A SRL consists of two parts, an optional protocol (string) and an address (string).
// The format of the SRL looks like this: `[<protocol>://]<address>`
// The protocol and the address have to contain only lowercase English characters.
// Protocol must not be empty if :// is present in the SRL.
// Address must not be empty.
//
// As an example, these are valid SRLs:
// - `http://foo`
// - `bar://baz`
// - `foobar`
//
// And these are invalid SRLs:
// - `http://foo1` (invalid character in address)
// - `asd://bar://` (invalid character in address)
// - `://baz` (empty protocol)
// - `01://baz` (invalid character in protocol)
//
// Create a struct `SRL` in a module named `srl`. Expose functions for parsing a SRL and getting
// its individual parts, but do not allow modifying the fields of `SRL` outside its module.
// Do not use regular expressions, SRLs can be easily parsed with a big of parsing logic.
//
// Hint: Put `#[derive(Debug, Eq, PartialEq)]` on top of `SRL` and `SRLValidationError`,
// so that asserts in tests work.

type Result = std::result::Result<SRL, SRLValidationError>;

#[derive(Debug, Eq, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
struct SRL {
    protocol: Option<String>,
    address: String,
}

impl SRL {
    fn new(srl: &str) -> Result {
        let parts: Vec<&str> = srl.splitn(2, "://").collect();

        let mut pidx = 0;
        let protocol = if parts.len() == 2 {
            let protocol = parts[0];
            match Self::is_part_valid(protocol) {
                Part::Empty => return Err(SRLValidationError::EmptyProtocol),
                Part::Invalid(c) => return Err(SRLValidationError::InvalidCharacterInProtocol(c)),
                Part::Valid => {}
            }
            pidx += 1;
            Some(String::from(protocol))
        } else {
            None
        };

        let address = parts[pidx];
        match Self::is_part_valid(address) {
            Part::Empty => Err(SRLValidationError::EmptyAddress),
            Part::Invalid(c) => Err(SRLValidationError::InvalidCharacterInAddress(c)),
            Part::Valid => Ok(Self {
                protocol,
                address: String::from(address),
            }),
        }
    }

    fn is_part_valid(p: &str) -> Part {
        if p.is_empty() {
            return Part::Empty;
        }

        for c in p.chars() {
            if !c.is_ascii_lowercase() {
                return Part::Invalid(c);
            }

            /* Alternative using `match` that Clippy told me to use the next one.
            if !match c {
                'a'..='z' => true,
                _ => false,
            } {
                return Part::Invalid(c);
            }
            */
            /* Alternative using `matches!` macro that Clippy told me to use the one currently
               used.
            if !matches!(c, 'a'..='z') {
                return Part::Invalid(c);
            }
            */
        }

        Part::Valid
    }

    fn get_protocol(&self) -> Option<&str> {
        /* I had  this:
        `if let Some(p) = &self.protocol.as_ref() {`
        instead of
        `if let Some(p) = self.protocol.as_ref() {`

        Both compiles, but the former works because of of "match ergonomics" and the fact that
        references are copyable.
        This is the explanation of the second and the former.

        The second:
        1. Type: The expression self.protocol.as_ref() produces a value of type Option<&String>.
        2. Matching: The if let takes ownership of this temporary Option wrapper (which is fine,
           it's just a disposable wrapper).
        3. Binding: It unwraps the Some. p is bound to the contents directly.
        4. Result: p is of type &String. This is exactly what you want to return.

        While the former:
        1. Type: You call .as_ref(), getting a temporary Option<&String>. You then add &, creating a
           reference to that temporary wrapper. The type is &Option<&String>.
        2. Matching: Rust sees you are matching on a reference (&Option).
        3. Match Ergonomics: Thanks to modern Rust features (Match Ergonomics), the compiler
           automatically dereferences the &Option to see if it matches Some.
        4. Double Reference: Because you matched on a reference, strictly speaking, p should be
           bound as a reference to the content (&&String).
        5. Copy Coercion: However, because the inner content is already a reference (&String) and
           shared references implement the Copy trait, the compiler essentially flattens this.
        6. Result: p ends up being &String.

        A shorter alternative is directly return:
        `self.protocol.as_ref().map(|s| s.as_str())` this is translates to:
        1. .as_ref() converts Option<String> -> Option<&String>
        2. .map() converts &String -> &str (via deref coercion)

        But thanks to the .as_deref method which takes Option<String> to return Option<&str> it
        borrows the Option an automatically performs a deref coercion on the content, then it goes
        straight to the string slice.

        Why do the first 2 approaches (if conditional ways) work if the returned type is
        Option<&String> and the method return type is Option<&str>, which are strictly different
        types?
        Because the Rust compiler applied an automatic coercion from &String to &str
        */
        self.protocol.as_deref()
    }

    fn get_address(&self) -> &str {
        &self.address
    }
}

#[derive(Debug, Eq, PartialEq)]
enum SRLValidationError {
    EmptyProtocol,
    EmptyAddress,
    InvalidCharacterInProtocol(char),
    InvalidCharacterInAddress(char),
}

enum Part {
    Valid,
    Empty,
    Invalid(char),
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::{SRLValidationError, SRL};

    #[test]
    fn empty_address() {
        assert_eq!(SRL::new(""), Err(SRLValidationError::EmptyAddress));
    }

    #[test]
    fn only_separator() {
        assert_eq!(SRL::new("://"), Err(SRLValidationError::EmptyProtocol));
    }

    #[test]
    fn empty_protocol() {
        assert_eq!(SRL::new("://foo"), Err(SRLValidationError::EmptyProtocol));
    }

    #[test]
    fn multiple_protocols() {
        assert_eq!(
            SRL::new("ab://bc://foo"),
            Err(SRLValidationError::InvalidCharacterInAddress(':'))
        );
    }

    #[test]
    fn invalid_protocol() {
        assert_eq!(
            SRL::new("bAc://foo"),
            Err(SRLValidationError::InvalidCharacterInProtocol('A'))
        );
        assert_eq!(
            SRL::new("a02://foo"),
            Err(SRLValidationError::InvalidCharacterInProtocol('0'))
        );
    }

    #[test]
    fn invalid_address_with_protocol() {
        assert_eq!(
            SRL::new("abc://fo1o"),
            Err(SRLValidationError::InvalidCharacterInAddress('1'))
        );
        assert_eq!(
            SRL::new("bar://fooBZcX"),
            Err(SRLValidationError::InvalidCharacterInAddress('B'))
        );
    }

    #[test]
    fn invalid_address_without_protocol() {
        assert_eq!(
            SRL::new("fo1o"),
            Err(SRLValidationError::InvalidCharacterInAddress('1'))
        );
        assert_eq!(
            SRL::new("fooBAc"),
            Err(SRLValidationError::InvalidCharacterInAddress('B'))
        );
    }

    #[test]
    fn invalid_protocol_and_address() {
        assert_eq!(
            SRL::new("bZcA://fo2o"),
            Err(SRLValidationError::InvalidCharacterInProtocol('Z'))
        );
        assert_eq!(
            SRL::new("a20://barBAZ"),
            Err(SRLValidationError::InvalidCharacterInProtocol('2'))
        );
    }

    #[test]
    fn invalid_char_emoji() {
        assert_eq!(
            SRL::new("asd://fo🙃o"),
            Err(SRLValidationError::InvalidCharacterInAddress('🙃'))
        );
    }

    #[test]
    fn no_protocol() {
        let srl = SRL::new("foobar").unwrap();
        assert_eq!(srl.get_protocol(), None);
        assert_eq!(srl.get_address(), "foobar");
    }

    #[test]
    fn protocol_and_scheme() {
        let srl = SRL::new("bar://foobar").unwrap();
        assert_eq!(srl.get_protocol(), Some("bar"));
        assert_eq!(srl.get_address(), "foobar");
    }
}
