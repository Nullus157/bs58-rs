use alphabet;
use error::DecodeError;

/// Decode given string to a vector of bytes using the [default alphabet][].
///
/// [default alphabet]: alphabet/constant.DEFAULT.html
pub fn decode<I: AsRef<[u8]>>(input: I) -> Result<Vec<u8>, DecodeError> {
    decode_with_alphabet(input, alphabet::DEFAULT)
}

/// Decode given string to a vector of bytes using the given alphabet.
pub fn decode_with_alphabet<I: AsRef<[u8]>>(input: I, alpha: &[u8; 58]) -> Result<Vec<u8>, DecodeError> {
    let input = input.as_ref();
    let zero = alpha[0];

    let alpha = {
        let mut rev = [0xFF; 256];
        for (i, &c) in alpha.iter().enumerate() {
            rev[c as usize] = i as u8;
        }
        rev
    };

    let mut bytes = Vec::with_capacity(input.len() / 8 * 6);

    for (i, c) in input.iter().enumerate() {
        let mut val = unsafe { *alpha.get_unchecked(*c as usize) as usize };
        if val == 0xFF {
            return Err(DecodeError::InvalidCharacter { character: *c as char, index: i })
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

    for c in input {
        if *c == zero {
            bytes.push(0);
        } else {
            break;
        }
    }

    bytes.reverse();
    Ok(bytes)
}

// Subset of test cases from https://github.com/cryptocoinjs/base-x/blob/master/test/fixtures.json
#[cfg(test)]
mod tests {
    use decode;

    #[test]
    fn tests() {
        for &(val, s) in super::super::TEST_CASES.iter() {
            assert_eq!(val.to_vec(), decode(s).unwrap());
        }
    }
}
