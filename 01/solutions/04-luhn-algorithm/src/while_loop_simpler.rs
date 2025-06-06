// This offers a simpler way to calculate it because the control digit is added to the computation
// and  it compares to 0 to assert if the number is valid.
// This approach doesn't appear on  https://en.wikipedia.org/wiki/Luhn_algorithm

fn luhn_algorithm(num: u64) -> bool {
    let mut num = num;
    let mut sum = 0;
    let mut alternate = false;

    while num != 0 {
        let mut digit = num % 10;
        num /= 10;

        if alternate {
            digit *= 2;
            if digit > 9 {
                digit -= 9
            }
        }

        alternate = !alternate;
        sum += digit
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
