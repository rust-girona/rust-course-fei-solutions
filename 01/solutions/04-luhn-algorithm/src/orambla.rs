//! Run this file with `cargo test --test 04_luhn_algorithm`.

// TODO: Implement the Luhn algorithm (https://en.wikipedia.org/wiki/Luhn_algorithm),
// which is used to check the validity of e.g. bank or credit card numbers.

fn luhn_algorithm(number: u64) -> bool {
    let mut sum = 0;
    let mut double = false;
    let mut n = number;

    // Process each digit from right to left
    while n > 0 {
        let mut digit = n % 10;

        if double {
            digit *= 2;
            if digit > 9 {
                digit = digit / 10 + digit % 10;
            }
        }

        sum += digit;
        double = !double;
        n /= 10;
    }

    sum % 10 == 0
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::luhn_algorithm;

    #[test]
    fn luhn_zero() {
        assert!(luhn_algorithm(0));
    }

    #[test]
    fn luhn_small_correct() {
        assert!(luhn_algorithm(18));
    }

    #[test]
    fn luhn_small_incorrect() {
        assert!(!luhn_algorithm(10));
    }

    #[test]
    fn luhn_correct() {
        assert!(luhn_algorithm(17893729974));
        assert!(luhn_algorithm(79927398713));
    }

    #[test]
    fn luhn_incorrect() {
        assert!(!luhn_algorithm(17893729975));
        assert!(!luhn_algorithm(17893729976));
        assert!(!luhn_algorithm(17893729977));
        assert!(!luhn_algorithm(123456));
    }
}
