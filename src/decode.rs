use { alphabet, DecodeError };

/// A builder for setting up the alphabet and output of a base58 decode.
#[allow(missing_debug_implementations)]
pub struct DecodeBuilder<'a, I: AsRef<[u8]>> {
    input: I,
    alpha: &'a [u8; 58],
}

impl<'a, I: AsRef<[u8]>> DecodeBuilder<'a, I> {
    /// Change the alphabet that will be used for decoding.
    pub fn with_alphabet(self, alpha: &[u8; 58]) -> DecodeBuilder<I> {
        DecodeBuilder { input: self.input, alpha: alpha }
    }

    /// Decode into a new vector of bytes.
    pub fn into_vec(self) -> Result<Vec<u8>, DecodeError> {
        let input = self.input.as_ref();
        let mut output = vec![0; (input.len() / 8 + 1) * 6];
        decode_into(input, &mut output, self.alpha)
            .map(|len| { output.truncate(len); output })
    }

    /// Decode into the given byte slice.
    /// Returns the length written into the byte slice.
    pub fn into<O: AsMut<[u8]>>(self, mut output: O) -> Result<usize, DecodeError> {
        decode_into(self.input.as_ref(), output.as_mut(), self.alpha)
    }
}

/// Setup decoder for the given string using the [default alphabet][].
/// [default alphabet]: alphabet/constant.DEFAULT.html
pub fn decode<I: AsRef<[u8]>>(input: I) -> DecodeBuilder<'static, I> {
    DecodeBuilder { input: input, alpha: alphabet::DEFAULT }
}

/// Decode given string into given byte slice using the given alphabet.
///
/// Returns the length written into the byte slice.
pub fn decode_into(input: &[u8], mut output: &mut [u8], alpha: &[u8; 58]) -> Result<usize, DecodeError> {
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
    use { decode, DecodeError };

    #[test]
    fn tests() {
        for &(val, s) in super::super::TEST_CASES.iter() {
            assert_eq!(val.to_vec(), decode(s).into_vec().unwrap());
        }
    }

    #[test]
    fn test_small_buffer_err() {
        let mut output = [0; 2];
        assert_eq!(decode("a3gV").into(&mut output), Err(DecodeError::BufferTooSmall));
    }
}
