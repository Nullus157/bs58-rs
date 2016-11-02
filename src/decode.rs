use std::error::Error;
use std::fmt;

use alphabet;

/// A trait for decoding Base58 encoded values to a vector of bytes.
pub trait FromBase58 {
    /// Decode `self` to a vector of bytes using the [default alphabet][].
    ///
    /// [default alphabet]: alphabet/constant.DEFAULT.html
    fn from_base58(&self) -> Result<Vec<u8>, FromBase58Error>;

    /// Decode `self` to a vector of bytes using the given alphabet.
    fn from_base58_with_alphabet(&self, alpha: &[u8; 58]) -> Result<Vec<u8>, FromBase58Error>;
}

/// Errors that could occur when decoding a Base58 encoded string.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FromBase58Error {
    /// The input contained a character that was not part of the current Base58
    /// alphabet.
    InvalidCharacter {
        /// The unexpected character.
        character: u8,
        /// The index in the input string the character was at.
        index: usize,
    }
}

impl FromBase58 for str {
    fn from_base58(&self) -> Result<Vec<u8>, FromBase58Error> {
        self.from_base58_with_alphabet(alphabet::DEFAULT)
    }

    fn from_base58_with_alphabet(&self, alpha: &[u8; 58]) -> Result<Vec<u8>, FromBase58Error> {
        let zero = alpha[0];

        let alpha = {
            let mut rev = [0xFF; 256];
            for i in 0..58 {
                rev[alpha[i] as usize] = i as u8;
            }
            rev
        };

        let mut bytes = Vec::with_capacity(self.len() / 8 * 6);

        for (i, c) in self.bytes().enumerate() {
            let mut val = unsafe { *alpha.get_unchecked(c as usize) as usize };
            if val == 0xFF {
                return Err(FromBase58Error::InvalidCharacter { character: c, index: i })
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

impl Error for FromBase58Error {
    fn description(&self) -> &str {
        match *self {
            FromBase58Error::InvalidCharacter { .. } =>
                "base58 encoded string contained an invalid character"
        }
    }
}

impl fmt::Display for FromBase58Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FromBase58Error::InvalidCharacter { character, index } => write!(f,
                "provided string contained invalid character {} at position {}",
                character,
                index)
        }
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
