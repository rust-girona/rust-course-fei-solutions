// This offers a simpler way to calculate it because the control digit is added to the computation
// and  it compares to 0 to assert if the number is valid.
// This approach doesn't appear on  https://en.wikipedia.org/wiki/Luhn_algorithm

fn luhn_algorithm(num: u64) -> bool {
    let num_text = num.to_string();
    let chars = num_text.chars().rev();

    let calculation = chars.enumerate().fold(0, |acc, (i, v)| {
        let mut v =
            v.to_digit(10)
                .expect("this is always a digit from numb which can never fail") as u64;

        if (i % 2) != 0 {
            v *= 2;
            if v > 9 {
                v -= 9
            }
        }

        acc + v
    });

    calculation % 10 == 0
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
