use alphabet;

/// A trait for decoding Base58 encoded values to a vector of bytes.
pub trait FromBase58 {
    /// Decode `self` to a vector of bytes using the [default alphabet][].
    ///
    /// [default alphabet]: alphabet/constant.DEFAULT.html
    fn from_base58(&self) -> Result<Vec<u8>, String>;

    /// Decode `self` to a vector of bytes using the given alphabet.
    fn from_base58_with_alphabet(&self, alpha: &[u8; 58]) -> Result<Vec<u8>, String>;
}

impl FromBase58 for str {
    fn from_base58(&self) -> Result<Vec<u8>, String> {
        self.from_base58_with_alphabet(alphabet::DEFAULT)
    }

    fn from_base58_with_alphabet(&self, alpha: &[u8; 58]) -> Result<Vec<u8>, String> {
        let zero = alpha[0];

        let alpha = {
            let mut rev = [0xFF; 256];
            for i in 0..58 {
                rev[alpha[i] as usize] = i as u8;
            }
            rev
        };

        let mut bytes = Vec::with_capacity(self.len() / 8 * 6);

        for c in self.bytes() {
            let mut val = unsafe { *alpha.get_unchecked(c as usize) as usize };
            if val == 0xFF {
                return Err(format!("unexpected utf8 byte '{}'", c));
            } else {
                for byte in &mut bytes {
                    val += (*byte as usize) * 58;
                    *byte = (val & 0xFF) as u8;
                    val >>= 8;
                }

                while val > 0 {
                    bytes.push((val & 0xff) as u8);
                    val >>= 8
                }
            }
        }

        for c in self.bytes() {
            if c == zero {
                bytes.push(0);
            } else {
                break;
            }
        }

        bytes.reverse();
        Ok(bytes)
    }
}

// Subset of test cases from https://github.com/cryptocoinjs/base-x/blob/master/test/fixtures.json
#[cfg(test)]
mod tests {
    use FromBase58;

    #[test]
    fn tests() {
        for &(val, s) in super::super::TEST_CASES.iter() {
            assert_eq!(val.to_vec(), s.from_base58().unwrap());
        }
    }
}
