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
    with_check: bool,
    expected_ver: Option<u8>
}

impl<'a, I: AsRef<[u8]>> DecodeBuilder<'a, I> {
    /// Setup decoder for the given string using the given alphabet.
    /// Preferably use [`bs58::decode`](../fn.decode.html) instead of this
    /// directly.
    pub fn new(input: I, alpha: &'a [u8; 58]) -> DecodeBuilder<'a, I> {
        DecodeBuilder { input: input, alpha: alpha, with_check: false, expected_ver: None }
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
        DecodeBuilder
        {
            input: self.input,
            alpha,
            with_check: self.with_check,
            expected_ver: self.expected_ver
        }
    }

    /// Expect and check checksum when decoding.
    ///
    /// Option parameter for version byte. If provided, the
    /// version byte will be used in verification.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     vec![0x2d, 0x31],
    ///     bs58::decode("PWEu9GGN")
    ///         .with_check(None)
    ///         .into_vec().unwrap());
    /// ```
    #[cfg(feature = "check")]
    pub fn with_check(mut self, expected_ver: Option<u8>) -> DecodeBuilder<'a, I> {
        self.with_check = true;
        self.expected_ver = expected_ver;
        self
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
        match self.with_check {
          true  => decode_check_into(input, &mut output, self.alpha, self.expected_ver),
          false => decode_into(input, &mut output, self.alpha)
        }.map(|len| { output.truncate(len); output })
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
        match self.with_check {
            true  => decode_check_into(self.input.as_ref(), output.as_mut(), self.alpha, self.expected_ver),
            false => decode_into(self.input.as_ref(), output.as_mut(), self.alpha)
        }
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

/// Decode given input slice into given output slice using the given alphabet. Expects
/// and validates checksum bytes in input.
///
/// Option for version byte. If given, it is used to verify input.
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
/// let input = "PWEu9GGN";
/// let mut output = [0; 6];
/// let l = bs58::decode::decode_check_into(input.as_ref(), &mut output, bs58::alphabet::DEFAULT, None);
/// assert_eq!([0x2d, 0x31], output[..l.unwrap()]);
/// ```
#[cfg(feature = "check")]
pub fn decode_check_into(input: &[u8], output: &mut [u8], alpha: &[u8; 58], expected_ver: Option<u8>) -> Result<usize, DecodeError> {
    use sha2::{Sha256, Digest};
    use CHECKSUM_LEN;

    let decoded_len = decode_into(input, output, alpha)?;
    if decoded_len < CHECKSUM_LEN {
        return Err(DecodeError::NoChecksum)
    }
    let checksum_index = decoded_len - CHECKSUM_LEN;

    let expected_checksum = &output[checksum_index..decoded_len];

    let first_hash = Sha256::digest(&output[0..checksum_index]);
    let second_hash = Sha256::digest(&first_hash);
    let (checksum, _) = second_hash.split_at(CHECKSUM_LEN);

    match checksum == expected_checksum {
        true => {
            match expected_ver {
                Some(ver) => {
                    if output[0] == ver {
                        Ok(checksum_index)
                    }
                    else {
                        Err(DecodeError::InvalidVersion{ver: output[0], expected_ver: ver})
                    }
                }
                None => Ok(checksum_index)
            }

        },
        false => {
            let mut a: [u8; CHECKSUM_LEN] = Default::default();
            a.copy_from_slice(&checksum[..]);
            let mut b: [u8; CHECKSUM_LEN] = Default::default();
            b.copy_from_slice(&expected_checksum[..]);
            Err(DecodeError::InvalidChecksum{checksum:a, expected_checksum:b})
        }
    }
}


#[cfg(not(feature = "check"))]
pub fn decode_check_into(_input: &[u8], _output: &mut [u8], _alpha: &[u8; 58], _expected_ver: Option<u8>) -> Result<usize, DecodeError> {
    unreachable!("This function requires 'checksum' feature");
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

#[cfg(test)]
#[cfg(feature = "check")]
mod test_check{
    use decode;
    use decode::DecodeError;

    #[test]
    fn test_check(){
        for &(val, s) in super::super::CHECK_TEST_CASES.iter() {
            assert_eq!(val.to_vec(), decode(s).with_check(None).into_vec().unwrap());
        }

        for &(val, s) in super::super::CHECK_TEST_CASES[1..].iter() {
            assert_eq!(val.to_vec(), decode(s).with_check(Some(val[0])).into_vec().unwrap());
        }
    }

    #[test]
    fn test_check_ver_failed() {
        let d = decode(super::super::CHECK_TEST_CASES[6].1)
            .with_check(Some(0x01))
            .into_vec();

        assert!(d.is_err());
        if let DecodeError::InvalidVersion {ver: _, expected_ver: _} = d.unwrap_err() {}
        else {
            assert!(false, "Not expected variant")
        }
    }
}
