use { alphabet, DecodeError };

/// Decode given string to a vector of bytes using the [default alphabet][].
///
/// [default alphabet]: alphabet/constant.DEFAULT.html
pub fn decode<I: AsRef<[u8]>>(input: I) -> Result<Vec<u8>, DecodeError> {
    decode_with_alphabet(input, alphabet::DEFAULT)
}

/// Decode given string into given byte slice using the [default alphabet][].
///
/// Returns the length written into the byte slice.
///
/// [default alphabet]: alphabet/constant.DEFAULT.html
pub fn decode_into<I: AsRef<[u8]>, O: AsMut<[u8]>>(input: I, output: O) -> Result<usize, DecodeError> {
    decode_into_with_alphabet(input, output, alphabet::DEFAULT)
}

/// Decode given string to a vector of bytes using the given alphabet.
pub fn decode_with_alphabet<I: AsRef<[u8]>>(input: I, alpha: &[u8; 58]) -> Result<Vec<u8>, DecodeError> {
    let mut output = vec![0; (input.as_ref().len() / 8 + 1) * 6];
    decode_into_with_alphabet(input, &mut output, alpha)
        .map(|len| { output.truncate(len); output })
}

/// Decode given string into given byte slice using the given alphabet.
///
/// Returns the length written into the byte slice.
pub fn decode_into_with_alphabet<I: AsRef<[u8]>, O: AsMut<[u8]>>(input: I, mut output: O, alpha: &[u8; 58]) -> Result<usize, DecodeError> {
    let input = input.as_ref();
    let mut output = output.as_mut();
    let mut index = 0;
    let zero = alpha[0];

    let alpha = {
        let mut rev = [0xFF; 256];
        for (i, &c) in alpha.iter().enumerate() {
            rev[c as usize] = i as u8;
        }
        rev
    };

    for (i, c) in input.iter().enumerate() {
        let mut val = unsafe { *alpha.get_unchecked(*c as usize) as usize };
        if val == 0xFF {
            return Err(DecodeError::InvalidCharacter { character: *c as char, index: i })
        } else {
            for byte in &mut output[..index] {
                val += (*byte as usize) * 58;
                *byte = (val & 0xFF) as u8;
                val >>= 8;
            }

            while val > 0 {
                let byte = output.get_mut(index).ok_or(DecodeError::BufferTooSmall)?;
                *byte = (val & 0xFF) as u8;
                index += 1;
                val >>= 8
            }
        }
    }

    for c in input {
        if *c == zero {
            let byte = output.get_mut(index).ok_or(DecodeError::BufferTooSmall)?;
            *byte = 0;
            index += 1;
        } else {
            break;
        }
    }

    output[..index].reverse();
    Ok(index)
}

// Subset of test cases from https://github.com/cryptocoinjs/base-x/blob/master/test/fixtures.json
#[cfg(test)]
mod tests {
    use { decode, decode_into, DecodeError };

    #[test]
    fn tests() {
        for &(val, s) in super::super::TEST_CASES.iter() {
            assert_eq!(val.to_vec(), decode(s).unwrap());
        }
    }

    #[test]
    fn test_small_buffer_err() {
        let mut output = [0; 2];
        assert_eq!(decode_into("a3gV", &mut output), Err(DecodeError::BufferTooSmall));
    }
}
