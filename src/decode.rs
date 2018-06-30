//! Functions for decoding Base58 encoded strings.

pub use error::DecodeError;

/// A builder for setting up the alphabet and output of a base58 decode.
///
/// See the documentation for [`bs58::decode`](../fn.decode.html) for a more
/// high level view of how to use this.
#[allow(missing_debug_implementations)]
pub struct DecodeBuilder<'a, I: AsRef<[u8]>> {
    input: I,
    alpha: &'a [u8; 58],
}

impl<'a, I: AsRef<[u8]>> DecodeBuilder<'a, I> {
    /// Setup decoder for the given string using the given alphabet.
    /// Preferably use [`bs58::decode`](../fn.decode.html) instead of this
    /// directly.
    pub fn new(input: I, alpha: &'a [u8; 58]) -> DecodeBuilder<'a, I> {
        DecodeBuilder { input: input, alpha: alpha }
    }

    /// Change the alphabet that will be used for decoding.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     vec![0x60, 0x65, 0xe7, 0x9b, 0xba, 0x2f, 0x78],
    ///     bs58::decode("he11owor1d")
    ///         .with_alphabet(bs58::alphabet::RIPPLE)
    ///         .into_vec().unwrap());
    /// ```
    #[allow(needless_lifetimes)] // They're specified for nicer documentation
    pub fn with_alphabet<'b>(self, alpha: &'b [u8; 58]) -> DecodeBuilder<'b, I> {
        DecodeBuilder { input: self.input, alpha: alpha }
    }

    /// Decode into a new vector of bytes.
    ///
    /// See the documentation for [`bs58::decode`](../fn.decode.html) for an
    /// explanation of the errors that may occur.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     vec![0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58],
    ///     bs58::decode("he11owor1d").into_vec().unwrap());
    /// ```
    ///
    pub fn into_vec(self) -> Result<Vec<u8>, DecodeError> {
        let input = self.input.as_ref();
        let mut output = vec![0; (input.len() / 8 + 1) * 6];
        decode_into(input, &mut output, self.alpha)
            .map(|len| { output.truncate(len); output })
    }

    /// Decode into the given buffer.
    ///
    /// Returns the length written into the buffer, the rest of the bytes in
    /// the buffer will be untouched.
    ///
    /// See the documentation for [`bs58::decode`](../fn.decode.html) for an
    /// explanation of the errors that may occur.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut output = [0xFF; 10];
    /// assert_eq!(8, bs58::decode("he11owor1d").into(&mut output).unwrap());
    /// assert_eq!(
    ///     [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58, 0xFF, 0xFF],
    ///     output);
    /// ```
    pub fn into<O: AsMut<[u8]>>(self, mut output: O) -> Result<usize, DecodeError> {
        decode_into(self.input.as_ref(), output.as_mut(), self.alpha)
    }
}

/// Decode given string into given byte slice using the given alphabet.
///
/// Returns the length written into the byte slice, the rest of the bytes in
/// the slice will be left untouched.
///
/// This is the low-level implementation that the `DecodeBuilder` uses to
/// perform the decoding, it's very likely that the signature will change if
/// the major version changes.
///
/// # Examples
///
/// ```rust
/// let input = "he11owor1d";
/// let mut output = [0; 8];
/// bs58::decode::decode_into(input.as_ref(), &mut output, bs58::alphabet::DEFAULT);
/// assert_eq!([0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58], output);
/// ```
pub fn decode_into(input: &[u8], output: &mut [u8], alpha: &[u8; 58]) -> Result<usize, DecodeError> {
    let mut index = 0;
    let zero = alpha[0];

    let alpha = {
        let mut rev = [0xFF; 128];
        for (i, &c) in alpha.iter().enumerate() {
            rev[c as usize] = i as u8;
        }
        rev
    };

    for (i, c) in input.iter().enumerate() {
        if *c > 127 {
            return Err(DecodeError::NonAsciiCharacter { index: i });
        }
        let mut val = unsafe { *alpha.get_unchecked(*c as usize) as usize };
        if val == 0xFF {
            return Err(DecodeError::InvalidCharacter { character: *c as char, index: i });
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
    use decode;
    use decode::DecodeError;

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
