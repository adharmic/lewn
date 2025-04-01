pub fn compute_check_digit(sequence: &String) -> u32 {
    let sum = compute_checksum(sequence, true);
    return 10 - (sum % 10);
}

pub fn compute_valid_sequence(sequence: &String) -> String {
    let check_digit = compute_check_digit(sequence);
    if check_digit == 10 {
        return sequence.clone();
    }
    let valid_sequence = sequence.to_owned() + &check_digit.to_string();
    return valid_sequence;
}

pub fn compute_checksum(sequence: &String, even: bool) -> u32 {
    let reversed: Vec<char> = sequence.chars().rev().collect();
    let mut remainder = 1;
    if even {
        remainder = 0;
    }
    let mut sum: u32 = 0;
    for i in 0..reversed.len() {
        let digit = char::to_digit(reversed[i], 10).unwrap();
        if i % 2 == remainder {
            let doubled_digit = digit * 2;
            if doubled_digit > 9 {
                sum += 1 + doubled_digit % 10;
            } else {
                sum += doubled_digit;
            }
        } else {
            sum += digit;
        }
    }
    return sum;
}

pub fn validate_sequence(sequence: &String) -> bool {
    return compute_checksum(sequence, false) % 10 == 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_valid_check_digit() {
        let sequence = String::from("1234");
        assert_eq!(compute_check_digit(&sequence), 4);
    }

    #[test]
    fn should_generate_valid_luhn_number() {
        let sequence = String::from("1234");
        assert_eq!(compute_valid_sequence(&sequence), "12344");
    }

    #[test]
    fn should_generate_valid_checksum() {
        let sequence = String::from("1234");
        assert_eq!(compute_checksum(&sequence, false), 14);
    }

    #[test]
    fn should_validate_valid_luhn_number() {
        let sequence = String::from("12344");
        assert_eq!(validate_sequence(&sequence), true);
    }

    #[test]
    fn should_not_validate_invalid_luhn_number() {
        let sequence = String::from("1234");
        assert_eq!(validate_sequence(&sequence), false);
    }
}
