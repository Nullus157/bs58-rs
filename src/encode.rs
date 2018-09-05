//! Functions for encoding into Base58 encoded strings.

/// A builder for setting up the alphabet and output of a base58 encode.
#[allow(missing_debug_implementations)]
pub struct EncodeBuilder<'a, I: AsRef<[u8]>> {
    input: I,
    alpha: &'a [u8; 58],
    with_check: bool,
}

impl<'a, I: AsRef<[u8]>> EncodeBuilder<'a, I> {
    /// Setup encoder for the given string using the given alphabet.
    /// Preferably use [`bs58::encode`](../fn.encode.html) instead of this
    /// directly.
    pub fn new(input: I, alpha: &'a [u8; 58]) -> EncodeBuilder<'a, I> {
        EncodeBuilder { input: input, alpha, with_check: false}
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
    #[allow(needless_lifetimes)] // They're specified for nicer documentation
    pub fn with_alphabet<'b>(self, alpha: &'b [u8; 58]) -> EncodeBuilder<'b, I> {
        EncodeBuilder { input: self.input, alpha, with_check: false}
    }

    pub fn with_check(self) -> EncodeBuilder<'a, I> {
        EncodeBuilder
        {
            input: self.input,
            alpha: self.alpha,
            with_check: true,
        }
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

        let checksum_capacity = match self.with_check {
            true => 4,
            false => 0
        };

        let mut output = String::with_capacity((input.len() / 5 + 1) * 8 + checksum_capacity);
        match self.with_check {
            true => encode_check_into(input, &mut output, self.alpha),
            false => encode_into(input, &mut output, self.alpha)
        };
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
        match self.with_check {
            true => encode_check_into(self.input.as_ref(), output, self.alpha),
            false => encode_into(self.input.as_ref(), output, self.alpha),
        }
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
    _encode_into(input, None, output, alpha)
}

static EMPTY_SLICE: &[u8] = &[];

fn _encode_into(input: &[u8], input_checksum: Option<&[u8]>, output: &mut String, alpha: &[u8; 58]) {


    assert!(alpha.iter().all(|&c| c < 128));

    output.clear();
    let output = unsafe {
        // Writing directly to the bytes of this string is safe as above we have
        // verified that we are only going to be writing ASCII bytes, which is a
        // valid subset of UTF-8.
        //
        // We will also be temporarily pushing values in the range [0, 58)
        // before we transform these into the alphabet. These are also valid
        // UTF-8 bytes.
        output.as_mut_vec()
    };

    let input_checksum = match input_checksum {
        Some(checksum) => checksum,
        None => EMPTY_SLICE
    };

    for &val in input.iter().chain(input_checksum.iter()) {
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

    for &val in input.iter().chain(input_checksum.iter()) {
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

pub fn encode_check_into(input: &[u8], output: &mut String, alpha: &[u8; 58]) {
    use sha2::{Sha256, Digest};
    use CHECKSUM_LEN;

    let first_hash = Sha256::digest(input);
    let second_hash = Sha256::digest(&first_hash);

    let checksum = &second_hash[0..CHECKSUM_LEN];


    _encode_into(input, Some(checksum), output, alpha)
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

#[cfg(test)]
mod tests_check {
    use encode;

    #[test]
    fn tests() {
        for &(val, s) in super::super::CHECK_TEST_CASES.iter() {
            assert_eq!(s, encode(val).with_check().into_string())
        }
    }
}
