/// A trait for Base58 encoding bytes to an owned string.
pub trait ToBase58 {
    /// Base58 encode `self` to an owned string using the default alphabet.
    fn to_base58(&self) -> String;

    /// Base58 encode `self` to an owned string using the given alphabet.
    fn to_base58_with_alphabet(&self, alpha: &[u8; 58]) -> String;
}

impl ToBase58 for [u8] {
    fn to_base58(&self) -> String {
        self.to_base58_with_alphabet(super::DEFAULT_ALPHABET)
    }

    fn to_base58_with_alphabet(&self, alpha: &[u8; 58]) -> String {
        if self.len() == 0 {
            return "".to_owned();
        }

        let mut digits = Vec::with_capacity(self.len() / 5 * 8);
        for &val in self.iter() {
            let mut carry = val as usize;
            for digit in &mut digits {
                carry += *digit << 8;
                *digit = carry % 58;
                carry = carry / 58;
            }
            while carry > 0 {
                digits.push(carry % 58);
                carry = carry / 58;
            }
        }

        let mut string = String::with_capacity(self.len() / 5 * 8);
        for &val in self.iter() {
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
}

// Subset of test cases from https://github.com/cryptocoinjs/base-x/blob/master/test/fixtures.json
#[cfg(test)]
mod tests {
    use ToBase58;

    #[test]
    fn tests() {
        for &(val, s) in super::super::TEST_CASES.iter() {
            assert_eq!(s, val.to_base58());
        }
    }
}
