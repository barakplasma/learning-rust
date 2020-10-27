fn main() {
    validate(1);
}

fn validate(n: u64) -> bool {
    let cc = n.to_string().chars().filter_map(|digit| digit.to_digit(10)).collect::<Vec<u32>>();
    let mut sum = *cc.iter().last().unwrap();

    for (i, digit) in cc.iter().take(cc.iter().count() - 1).enumerate() {
        let mut current = *digit;
        if i % 2 == 0 {
            current *= 2;
        }
        if current > 9 {
            current -= 9;
        }

        sum += current;
    }

    return sum % 10 == 0;
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_123() {
        assert_eq!(validate(123), false);
    }
    #[test]
    fn test_validate_1() {
        assert_eq!(validate(1), false);
    }
    #[test]
    fn test_validate_2121() {
        assert_eq!(validate(2121), true);
    }
    #[test]
    fn test_validate_1230() {
        assert_eq!(validate(1230), true);
    }
    #[test]
    fn test_validate_8675309() {
        assert_eq!(validate(8675309), false);
    }
    #[test]
    fn test_validate_4111111111111111() {
        assert_eq!(validate(4111111111111111), true);
    }
    #[test]
    fn test_validate_26() {
        assert_eq!(validate(26), true);
    }
    #[test]
    fn test_validate_2626262626262626() {
        assert_eq!(validate(2626262626262626), true);
    }
    #[test]
    fn test_validate_91() {
        assert_eq!(validate(91), true);
    }
    #[test]
    fn test_validate_92() {
        assert_eq!(validate(92), false);
    }
    #[test]
    fn test_validate_912030() {
        assert_eq!(validate(912030), true);
    }
    #[test]
    fn test_validate_922030() {
        assert_eq!(validate(922030), false);
    }
}
