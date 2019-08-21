//! Functions for decoding Base58 encoded strings.

use std::fmt;

#[cfg(feature = "check")]
use CHECKSUM_LEN;

/// A builder for setting up the alphabet and output of a base58 decode.
///
/// See the documentation for [`bs58::decode`](../fn.decode.html) for a more
/// high level view of how to use this.
#[allow(missing_debug_implementations)]
pub struct DecodeBuilder<'a, I: AsRef<[u8]>> {
    input: I,
    alpha: &'a [u8; 58],
    check: bool,
    expected_ver: Option<u8>
}

/// Errors that could occur when decoding a Base58 encoded string.
#[deprecated(since = "0.2.5", note = "Use `bs58::decode::Error` instead")]
pub type DecodeError = Error;

/// A specialized [`Result`](std::result::Result) type for [`bs58::decode`](module@crate::decode)
pub type Result<T> = ::std::result::Result<T, Error>;

/// Errors that could occur when decoding a Base58 encoded string.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// The output buffer was too small to contain the entire input.
    BufferTooSmall,

    /// The input contained a character that was not part of the current Base58
    /// alphabet.
    InvalidCharacter {
        /// The unexpected character.
        character: char,
        /// The (byte) index in the input string the character was at.
        index: usize,
    },

    /// The input contained a multi-byte (or non-utf8) character which is
    /// unsupported by this Base58 decoder.
    NonAsciiCharacter {
        /// The (byte) index in the input string the start of the character was
        /// at.
        index: usize,
    },

    #[cfg(feature = "check")]
    /// The checksum did not match the payload bytes
    InvalidChecksum {
        ///The given checksum
        checksum: [u8; CHECKSUM_LEN],
        ///The checksum calculated for the payload
        expected_checksum: [u8; CHECKSUM_LEN]
    },

    #[cfg(feature = "check")]
    /// The checksum did not match the payload bytes
    InvalidVersion {
        ///The given checksum
        ver: u8,
        ///The checksum calculated for the payload
        expected_ver: u8
    },

    #[cfg(feature = "check")]
    ///Not enough bytes to have both a checksum and a payload (less than to CHECKSUM_LEN)
    NoChecksum,

    #[doc(hidden)]
    __NonExhaustive,
}

impl<'a, I: AsRef<[u8]>> DecodeBuilder<'a, I> {
    /// Setup decoder for the given string using the given alphabet.
    /// Preferably use [`bs58::decode`](../fn.decode.html) instead of this
    /// directly.
    pub fn new(input: I, alpha: &'a [u8; 58]) -> DecodeBuilder<'a, I> {
        DecodeBuilder { input: input, alpha: alpha, check: false, expected_ver: None }
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
            alpha: alpha,
            check: self.check,
            expected_ver: self.expected_ver
        }
    }

    /// Expect and check checksum using the [Base58Check][] algorithm when
    /// decoding.
    ///
    /// Optional parameter for version byte. If provided, the version byte will
    /// be used in verification.
    ///
    /// [Base58Check]: https://en.bitcoin.it/wiki/Base58Check_encoding
    ///
    /// # Features
    ///
    /// Requires the `check` feature flag to be active.
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
        self.check = true;
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
    pub fn into_vec(self) -> Result<Vec<u8>> {
        let mut output = vec![0; self.input.as_ref().len()];
        self.into(&mut output).map(|len| {
            output.truncate(len);
            output
        })
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
    pub fn into<O: AsMut<[u8]>>(self, mut output: O) -> Result<usize> {
        if self.check {
            #[cfg(feature = "check")]
            {
                decode_check_into(
                    self.input.as_ref(),
                    output.as_mut(),
                    self.alpha,
                    self.expected_ver,
                )
            }
            #[cfg(not(feature = "check"))]
            {
                unreachable!("This function requires 'check' feature")
            }
        } else {
            decode_into(self.input.as_ref(), output.as_mut(), self.alpha)
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
pub fn decode_into(input: &[u8], output: &mut [u8], alpha: &[u8; 58]) -> Result<usize> {
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
            return Err(Error::NonAsciiCharacter { index: i });
        }
        let mut val = unsafe { *alpha.get_unchecked(*c as usize) as usize };
        if val == 0xFF {
            return Err(Error::InvalidCharacter { character: *c as char, index: i });
        } else {
            for byte in &mut output[..index] {
                val += (*byte as usize) * 58;
                *byte = (val & 0xFF) as u8;
                val >>= 8;
            }

            while val > 0 {
                let byte = output.get_mut(index).ok_or(Error::BufferTooSmall)?;
                *byte = (val & 0xFF) as u8;
                index += 1;
                val >>= 8
            }
        }
    }

    for c in input {
        if *c == zero {
            let byte = output.get_mut(index).ok_or(Error::BufferTooSmall)?;
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
/// # Features
///
/// Requires the `check` feature flag to be active.
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
pub fn decode_check_into(input: &[u8], output: &mut [u8], alpha: &[u8; 58], expected_ver: Option<u8>) -> Result<usize> {
    use sha2::{Sha256, Digest};

    let decoded_len = decode_into(input, output, alpha)?;
    if decoded_len < CHECKSUM_LEN {
        return Err(Error::NoChecksum)
    }
    let checksum_index = decoded_len - CHECKSUM_LEN;

    let expected_checksum = &output[checksum_index..decoded_len];

    let first_hash = Sha256::digest(&output[0..checksum_index]);
    let second_hash = Sha256::digest(&first_hash);
    let (checksum, _) = second_hash.split_at(CHECKSUM_LEN);

    if checksum == expected_checksum {
        if let Some(ver) = expected_ver {
            if output[0] == ver {
                Ok(checksum_index)
            } else {
                Err(Error::InvalidVersion{ver: output[0], expected_ver: ver})
            }
        } else {
            Ok(checksum_index)
        }
    } else {
        let mut a: [u8; CHECKSUM_LEN] = Default::default();
        a.copy_from_slice(&checksum[..]);
        let mut b: [u8; CHECKSUM_LEN] = Default::default();
        b.copy_from_slice(&expected_checksum[..]);
        Err(Error::InvalidChecksum{checksum:a, expected_checksum:b})
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BufferTooSmall =>
                "buffer provided to decode base58 encoded string into was too small",
            Error::NonAsciiCharacter { .. } =>
                "base58 encoded string contained a non-ascii character",
            Error::InvalidCharacter { .. } =>
                "base58 encoded string contained an invalid character",
            #[cfg(feature = "check")]
            Error::InvalidChecksum { .. } =>
                "base58 decode check did not match payload checksum with expected checksum",
            #[cfg(feature = "check")]
            Error::InvalidVersion { .. } =>
                "base58 decode check did not match payload version with expected version",
            #[cfg(feature = "check")]
            Error::NoChecksum { .. } =>
                "base58 encoded string does not contained enough bytes to have a checksum",
            Error::__NonExhaustive => unreachable!(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BufferTooSmall => write!(f,
                "buffer provided to decode base58 encoded string into was too small"),
            Error::InvalidCharacter { character, index } => write!(f,
                "provided string contained invalid character {:?} at byte {}",
                character,
                index),
            Error::NonAsciiCharacter { index } => write!(f,
                "provided string contained non-ascii character starting at byte {}",
                index),
            #[cfg(feature = "check")]
            Error::InvalidChecksum { checksum, expected_checksum } => write!(f,
                "invalid checksum, calculated checksum: '{:?}', expected checksum: {:?}",
                checksum,
                expected_checksum),
            #[cfg(feature = "check")]
            Error::InvalidVersion { ver, expected_ver } => write!(f,
                "invalid version, payload version: '{:?}', expected version: {:?}",
                ver,
                expected_ver),
            #[cfg(feature = "check")]
            Error::NoChecksum => write!(f,
                "provided string is too small to contain a checksum"),
            Error::__NonExhaustive => unreachable!(),
        }
    }
}
// Subset of test cases from https://github.com/cryptocoinjs/base-x/blob/master/test/fixtures.json
#[cfg(test)]
mod tests {
    use decode;
    use decode::Error;

    #[test]
    fn tests() {
        for &(val, s) in super::super::TEST_CASES.iter() {
            assert_eq!(val.to_vec(), decode(s).into_vec().unwrap());
        }
    }

    #[test]
    fn test_small_buffer_err() {
        let mut output = [0; 2];
        assert_eq!(decode("a3gV").into(&mut output), Err(Error::BufferTooSmall));
    }

    #[test]
    fn test_invalid_char() {
        let sample = "123456789abcd!efghij";

        assert_eq!(
            decode(sample).into_vec().unwrap_err(),
            Error::InvalidCharacter { character: '!', index: 13 });
    }
}

#[cfg(test)]
#[cfg(feature = "check")]
mod test_check{
    use decode;
    use decode::Error;

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
        let d = decode("K5zqBMZZTzUbAZQgrt4")
            .with_check(Some(0x01))
            .into_vec();

        assert!(d.is_err());
        assert_matches!(d.unwrap_err(), Error::InvalidVersion { .. });
    }
}
