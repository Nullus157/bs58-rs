//! Functions for encoding into Base58 encoded strings.

/// A builder for setting up the alphabet and output of a base58 encode.
#[allow(missing_debug_implementations)]
pub struct EncodeBuilder<'a, I: AsRef<[u8]>> {
    input: I,
    alpha: &'a [u8; 58],
}

impl<'a, I: AsRef<[u8]>> EncodeBuilder<'a, I> {
    /// Setup encoder for the given string using the given alphabet.
    /// Preferably use [`bs58::encode`](../fn.encode.html) instead of this
    /// directly.
    pub fn new(input: I, alpha: &'a [u8; 58]) -> EncodeBuilder<'a, I> {
        EncodeBuilder { input: input, alpha: alpha }
    }

    /// Change the alphabet that will be used for encoding.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let input = [0x60, 0x65, 0xe7, 0x9b, 0xba, 0x2f, 0x78];
    /// assert_eq!(
    ///     "he11owor1d",
    ///     bs58::encode(input)
    ///         .with_alphabet(bs58::alphabet::RIPPLE)
    ///         .into_string());
    /// ```
    pub fn with_alphabet(self, alpha: &[u8; 58]) -> EncodeBuilder<I> {
        EncodeBuilder { input: self.input, alpha: alpha }
    }

    /// Encode into a new owned string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
    /// assert_eq!("he11owor1d", bs58::encode(input).into_string());
    /// ```
    pub fn into_string(self) -> String {
        let input = self.input.as_ref();
        let mut output = String::with_capacity((input.len() / 5 + 1) * 8);
        encode_into(input, &mut output, self.alpha);
        output
    }

    /// Encode into the given string, any existing data will be cleared.
    ///
    /// If the given string does not have enough capacity for the encoded
    /// version of the data it will be reallocated as necessary.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
    /// let mut output = "goodbye world".to_owned();
    /// bs58::encode(input).into(&mut output);
    /// assert_eq!("he11owor1d", output);
    /// ```
    pub fn into(self, output: &mut String) {
        encode_into(self.input.as_ref(), output, self.alpha);
    }
}

/// Encode given bytes into given string using the given alphabet, any existing
/// data will be cleared.
///
/// This is the low-level implementation that the `EncodeBuilder` uses to
/// perform the encoding, it's very likely that the signature will change if
/// the major version changes.
///
/// # Examples
///
/// ```rust
/// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
/// let mut output = "goodbye world".to_owned();
/// bs58::encode::encode_into(&input[..], &mut output, bs58::alphabet::DEFAULT);
/// assert_eq!("he11owor1d", output)
/// ```
pub fn encode_into(input: &[u8], output: &mut String, alpha: &[u8; 58]) {
    output.clear();
    let mut output = unsafe { output.as_mut_vec() };

    for &val in input.iter() {
        let mut carry = val as usize;
        for byte in &mut output[..] {
            carry += (*byte as usize) << 8;
            *byte = (carry % 58) as u8;
            carry /= 58;
        }
        while carry > 0 {
            output.push((carry % 58) as u8);
            carry /= 58;
        }
    }

    for &val in input.iter() {
        if val == 0 {
            output.push(0);
        } else {
            break;
        }
    }

    for val in &mut output[..] {
        *val = alpha[*val as usize];
    }

    output.reverse();
}

// Subset of test cases from https://github.com/cryptocoinjs/base-x/blob/master/test/fixtures.json
#[cfg(test)]
mod tests {
    use encode;

    #[test]
    fn tests() {
        for &(val, s) in super::super::TEST_CASES.iter() {
            assert_eq!(s, encode(val).into_string())
        }
    }
}
