// This follows one of the ways to calculate it from https://en.wikipedia.org/wiki/Luhn_algorithm

fn luhn_algorithm(num: u64) -> bool {
    let num_text = num.to_string();
    let mut chars = num_text.chars().rev();
    let check_digit = chars
        .next()
        .expect("num always have at least one digit")
        .to_digit(10)
        .expect("this is a digit from num which can never fail") as u64;

    let calculation = chars.enumerate().fold(0, |acc, (i, v)| {
        let mut v =
            v.to_digit(10)
                .expect("this is always a digit from numb which can never fail") as u64;

        if (i % 2) == 0 {
            v *= 2;
            if v > 9 {
                v -= 9
            }
        }

        acc + v
    });

    (10 - (calculation % 10)) % 10 == check_digit
}

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
