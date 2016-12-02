use alphabet;

/// Base58 encode given bytes to an owned string using the [default alphabet][].
///
/// [default alphabet]: alphabet/constant.DEFAULT.html
pub fn encode<S: AsRef<[u8]>>(input: S) -> String {
    encode_with_alphabet(input, alphabet::DEFAULT)
}

/// Base58 encode given bytes to an owned string using the given alphabet.
pub fn encode_with_alphabet<S: AsRef<[u8]>>(input: S, alpha: &[u8; 58]) -> String {
    let input = input.as_ref();

    if input.len() == 0 {
        return "".to_owned();
    }

    let mut digits = Vec::with_capacity(input.len() / 5 * 8);
    for &val in input.iter() {
        let mut carry = val as usize;
        for digit in &mut digits {
            carry += *digit << 8;
            *digit = carry % 58;
            carry /= 58;
        }
        while carry > 0 {
            digits.push(carry % 58);
            carry /= 58;
        }
    }

    let mut string = String::with_capacity(input.len() / 5 * 8);
    for &val in input.iter() {
        if val == 0 {
            string.push(alpha[0] as char);
        } else {
            break;
        }
    }
    for digit in digits.into_iter().rev() {
        string.push(alpha[digit] as char)
    }

    string
}

// Subset of test cases from https://github.com/cryptocoinjs/base-x/blob/master/test/fixtures.json
#[cfg(test)]
mod tests {
    use encode;

    #[test]
    fn tests() {
        for &(val, s) in super::super::TEST_CASES.iter() {
            assert_eq!(s, encode(val))
        }
    }
}
