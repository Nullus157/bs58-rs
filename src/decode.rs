//! Functions for decoding Base58 encoded strings.

use core::fmt;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::Check;
#[cfg(any(feature = "check", feature = "cb58"))]
use crate::CHECKSUM_LEN;

use crate::Alphabet;

/// A builder for setting up the alphabet and output of a base58 decode.
///
/// See the documentation for [`bs58::decode`](crate::decode()) for a more
/// high level view of how to use this.
#[allow(missing_debug_implementations)]
pub struct DecodeBuilder<'a, I: AsRef<[u8]>> {
    input: I,
    alpha: &'a Alphabet,
    check: Check,
}

/// A specialized [`Result`](core::result::Result) type for [`bs58::decode`](module@crate::decode)
pub type Result<T> = core::result::Result<T, Error>;

/// Errors that could occur when decoding a Base58 encoded string.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
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

    #[cfg(any(feature = "check", feature = "cb58"))]
    /// The checksum did not match the payload bytes
    InvalidChecksum {
        ///The given checksum
        checksum: [u8; CHECKSUM_LEN],
        ///The checksum calculated for the payload
        expected_checksum: [u8; CHECKSUM_LEN],
    },

    #[cfg(any(feature = "check", feature = "cb58"))]
    /// The version did not match the payload bytes
    InvalidVersion {
        ///The given version
        ver: u8,
        ///The expected version
        expected_ver: u8,
    },

    #[cfg(any(feature = "check", feature = "cb58"))]
    ///Not enough bytes to have both a checksum and a payload (less than to CHECKSUM_LEN)
    NoChecksum,
}

/// Represents a buffer that can be decoded into. See [`DecodeBuilder::into`] and the provided
/// implementations for more details.
pub trait DecodeTarget {
    /// Decodes into this buffer, provides the maximum length for implementations that wish to
    /// preallocate space, along with a function that will write bytes into the buffer and return
    /// the length written to it.
    fn decode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize>;
}

impl<T: DecodeTarget + ?Sized> DecodeTarget for &mut T {
    fn decode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        T::decode_with(self, max_len, f)
    }
}

#[cfg(feature = "alloc")]
impl DecodeTarget for Vec<u8> {
    fn decode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        let original = self.len();
        self.resize(original + max_len, 0);
        let len = f(&mut self[original..])?;
        self.truncate(original + len);
        Ok(len)
    }
}

impl DecodeTarget for [u8] {
    fn decode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        let _ = max_len;
        f(&mut *self)
    }
}

impl<const N: usize> DecodeTarget for [u8; N] {
    fn decode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        let _ = max_len;
        f(&mut *self)
    }
}

impl<'a, I: AsRef<[u8]>> DecodeBuilder<'a, I> {
    /// Setup decoder for the given string using the given alphabet.
    /// Preferably use [`bs58::decode`](crate::decode()) instead of this directly.
    pub fn new(input: I, alpha: &'a Alphabet) -> DecodeBuilder<'a, I> {
        DecodeBuilder {
            input,
            alpha,
            check: Check::Disabled,
        }
    }

    /// Setup decoder for the given string using default prepared alphabet.
    pub(crate) fn from_input(input: I) -> DecodeBuilder<'static, I> {
        DecodeBuilder {
            input,
            alpha: Alphabet::DEFAULT,
            check: Check::Disabled,
        }
    }

    /// Change the alphabet that will be used for decoding.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     vec![0x60, 0x65, 0xe7, 0x9b, 0xba, 0x2f, 0x78],
    ///     bs58::decode("he11owor1d")
    ///         .with_alphabet(bs58::Alphabet::RIPPLE)
    ///         .into_vec()?);
    /// # Ok::<(), bs58::decode::Error>(())
    /// ```
    pub fn with_alphabet(self, alpha: &'a Alphabet) -> DecodeBuilder<'a, I> {
        DecodeBuilder { alpha, ..self }
    }

    /// Expect and check checksum using the [Base58Check][] algorithm when
    /// decoding.
    ///
    /// Optional parameter for version byte. If provided, the version byte will
    /// be used in verification.
    ///
    /// [Base58Check]: https://en.bitcoin.it/wiki/Base58Check_encoding
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     vec![0x2d, 0x31],
    ///     bs58::decode("PWEu9GGN")
    ///         .with_check(None)
    ///         .into_vec()?);
    /// # Ok::<(), bs58::decode::Error>(())
    /// ```
    #[cfg(feature = "check")]
    pub fn with_check(self, expected_ver: Option<u8>) -> DecodeBuilder<'a, I> {
        let check = Check::Enabled(expected_ver);
        DecodeBuilder { check, ..self }
    }

    /// Expect and check checksum using the [CB58][] algorithm when
    /// decoding.
    ///
    /// Optional parameter for version byte. If provided, the version byte will
    /// be used in verification.
    ///
    /// [CB58]: https://support.avax.network/en/articles/4587395-what-is-cb58
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     vec![0x2d, 0x31],
    ///     bs58::decode("PWHVMzdR")
    ///         .as_cb58(None)
    ///         .into_vec()?);
    /// # Ok::<(), bs58::decode::Error>(())
    /// ```
    #[cfg(feature = "cb58")]
    pub fn as_cb58(self, expected_ver: Option<u8>) -> DecodeBuilder<'a, I> {
        let check = Check::CB58(expected_ver);
        DecodeBuilder { check, ..self }
    }

    /// Decode into a new vector of bytes.
    ///
    /// See the documentation for [`bs58::decode`](crate::decode()) for an
    /// explanation of the errors that may occur.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     vec![0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58],
    ///     bs58::decode("he11owor1d").into_vec()?);
    /// # Ok::<(), bs58::decode::Error>(())
    /// ```
    ///
    #[cfg(feature = "alloc")]
    pub fn into_vec(self) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        self.into(&mut output)?;
        Ok(output)
    }

    /// Decode into the given buffer.
    ///
    /// Returns the length written into the buffer.
    ///
    /// If the buffer is resizeable it will be extended and the new data will be written to the end
    /// of it.
    ///
    /// If the buffer is not resizeable bytes will be written from the beginning and bytes after
    /// the final encoded byte will not be touched.
    ///
    /// See the documentation for [`bs58::decode`](crate::decode()) for an
    /// explanation of the errors that may occur.
    ///
    /// # Examples
    ///
    /// ## `Vec<u8>`
    ///
    /// ```rust
    /// let mut output = b"hello ".to_vec();
    /// assert_eq!(5, bs58::decode("EUYUqQf").into(&mut output)?);
    /// assert_eq!(b"hello world", output.as_slice());
    /// # Ok::<(), bs58::decode::Error>(())
    /// ```
    ///
    /// ## `&mut [u8]`
    ///
    /// ```rust
    /// let mut output = b"hello ".to_owned();
    /// assert_eq!(5, bs58::decode("EUYUqQf").into(&mut output)?);
    /// assert_eq!(b"world ", output.as_ref());
    /// # Ok::<(), bs58::decode::Error>(())
    /// ```
    pub fn into(self, mut output: impl DecodeTarget) -> Result<usize> {
        let max_decoded_len = self.input.as_ref().len();
        match self.check {
            Check::Disabled => output.decode_with(max_decoded_len, |output| {
                decode_into(self.input.as_ref(), output, self.alpha)
            }),
            #[cfg(feature = "check")]
            Check::Enabled(expected_ver) => output.decode_with(max_decoded_len, |output| {
                decode_check_into(self.input.as_ref(), output, self.alpha, expected_ver)
            }),
            #[cfg(feature = "cb58")]
            Check::CB58(expected_ver) => output.decode_with(max_decoded_len, |output| {
                decode_cb58_into(self.input.as_ref(), output, self.alpha, expected_ver)
            }),
        }
    }
}


fn alpha_decode(index: usize, input_char: u8, alpha: &Alphabet) -> Result<u8> {
    if input_char > 127 {
        return Err(Error::NonAsciiCharacter { index });
    };
    let val = alpha.decode[input_char as usize];
    if val >= 58 {
        return Err(Error::InvalidCharacter {
            character: input_char as char,
            index,
        });
    }
    return Ok(val);
}

fn decode_into(input: &[u8], output: &mut [u8], alpha: &Alphabet) -> Result<usize> {
    let mut index = 0;
    let zero = alpha.encode[0];

    for (_, _) in input.iter().enumerate().take_while(|(_, c)| **c == zero) {
        let byte = output.get_mut(index).ok_or(Error::BufferTooSmall)?;
        *byte = 0;
        index += 1;
    }

    let index_0 = index;
    let input_len = input.len() - index_0;

    if input_len > 0 && input_len <= 10 {
        let mut output_uint = 0u64;
        for (i, c) in input.iter().enumerate().skip(index_0) {
            let val = alpha_decode(i, *c, alpha)? as u64;
            output_uint = 58 * output_uint + val;
        }
        while output_uint > 0 {
            let byte = output.get_mut(index).ok_or(Error::BufferTooSmall)?;
            *byte = output_uint as u8;
            index += 1;
            output_uint >>= 8
        }
        output[index_0..index].reverse();
    } else if input_len <= 21 {
        let mut output_uint = 0u128;
        for (i, c) in input.iter().enumerate().skip(index_0) {
            let val = alpha_decode(i, *c, alpha)? as u128;
            output_uint = 58 * output_uint + val;
        }
        while output_uint > 0 {
            let byte = output.get_mut(index).ok_or(Error::BufferTooSmall)?;
            *byte = output_uint as u8;
            index += 1;
            output_uint >>= 8
        }
        output[index_0..index].reverse();
    } else if input_len <= 43 {
        let mut output_uints = [0u64; 4];
        let mut ll_index = 0;
        for (i, c) in input.iter().enumerate().skip(index_0) {
            let mut val = alpha_decode(i, *c, alpha)? as u128;

            for ll in &mut output_uints[..ll_index] {
                val += *ll as u128 * 58;
                *ll = val as u64;
                val >>= 64;
            }
            while val > 0 {
                let ll = output_uints.get_mut(ll_index).expect("Base58 input under 43 chars fit into [u64;4]");
                *ll = val as u64;
                ll_index += 1;
                val >>= 64
            }
        }
        output_uints.reverse();
        let mut leading_0 = true;
        for ll in output_uints {
            for be_byte in ll.to_be_bytes() {
                if leading_0 && be_byte == 0 {
                    continue;
                } else {
                    leading_0 = false;
                }
                let byte = output.get_mut(index).ok_or(Error::BufferTooSmall)?;
                *byte = be_byte;
                index += 1;
            }
        }
    } else {
        let mut output_uints: Vec<u64> = Vec::with_capacity(1 + (7_323 * input_len) / 80_000 ); // [0u64; 4];
        let mut ll_index = 0;
        for (i, c) in input.iter().enumerate().skip(index_0) {
            let mut val = alpha_decode(i, *c, alpha)? as u128;
            for ll in &mut output_uints[..ll_index] {
                val += *ll as u128 * 58;
                *ll = val as u64;
                val >>= 64;
            }
            while val > 0 {
                ll_index += 1;
                output_uints.push(val as u64);
                val >>= 64
            }
        }
        output_uints.reverse();
        let mut leading_0 = true;
        for ll in output_uints {
            for be_byte in ll.to_be_bytes() {
                if leading_0 && be_byte == 0 {
                    continue;
                } else {
                    leading_0 = false;
                }
                let byte = output.get_mut(index).ok_or(Error::BufferTooSmall)?;
                *byte = be_byte;
                index += 1;
            }
        }
    }
    Ok(index)
}

#[cfg(feature = "check")]
fn decode_check_into(
    input: &[u8],
    output: &mut [u8],
    alpha: &Alphabet,
    expected_ver: Option<u8>,
) -> Result<usize> {
    use sha2::{Digest, Sha256};

    let decoded_len = decode_into(input, output, alpha)?;
    if decoded_len < CHECKSUM_LEN {
        return Err(Error::NoChecksum);
    }
    let checksum_index = decoded_len - CHECKSUM_LEN;

    let expected_checksum = &output[checksum_index..decoded_len];

    let first_hash = Sha256::digest(&output[0..checksum_index]);
    let second_hash = Sha256::digest(first_hash);
    let (checksum, _) = second_hash.split_at(CHECKSUM_LEN);

    if checksum == expected_checksum {
        if let Some(ver) = expected_ver {
            if output[0] == ver {
                Ok(checksum_index)
            } else {
                Err(Error::InvalidVersion {
                    ver: output[0],
                    expected_ver: ver,
                })
            }
        } else {
            Ok(checksum_index)
        }
    } else {
        let mut a: [u8; CHECKSUM_LEN] = Default::default();
        a.copy_from_slice(checksum);
        let mut b: [u8; CHECKSUM_LEN] = Default::default();
        b.copy_from_slice(expected_checksum);
        Err(Error::InvalidChecksum {
            checksum: a,
            expected_checksum: b,
        })
    }
}

#[cfg(feature = "cb58")]
fn decode_cb58_into(
    input: &[u8],
    output: &mut [u8],
    alpha: &Alphabet,
    expected_ver: Option<u8>,
) -> Result<usize> {
    use sha2::{Digest, Sha256};

    let decoded_len = decode_into(input, output, alpha)?;
    if decoded_len < CHECKSUM_LEN {
        return Err(Error::NoChecksum);
    }
    let checksum_index = decoded_len - CHECKSUM_LEN;

    let expected_checksum = &output[checksum_index..decoded_len];

    let hash = Sha256::digest(&output[0..checksum_index]);
    let (_, checksum) = hash.split_at(hash.len() - CHECKSUM_LEN);

    if checksum == expected_checksum {
        if let Some(ver) = expected_ver {
            if output[0] == ver {
                Ok(checksum_index)
            } else {
                Err(Error::InvalidVersion {
                    ver: output[0],
                    expected_ver: ver,
                })
            }
        } else {
            Ok(checksum_index)
        }
    } else {
        let mut a: [u8; CHECKSUM_LEN] = Default::default();
        a.copy_from_slice(checksum);
        let mut b: [u8; CHECKSUM_LEN] = Default::default();
        b.copy_from_slice(expected_checksum);
        Err(Error::InvalidChecksum {
            checksum: a,
            expected_checksum: b,
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BufferTooSmall => write!(
                f,
                "buffer provided to decode base58 encoded string into was too small"
            ),
            Error::InvalidCharacter { character, index } => write!(
                f,
                "provided string contained invalid character {:?} at byte {}",
                character, index
            ),
            Error::NonAsciiCharacter { index } => write!(
                f,
                "provided string contained non-ascii character starting at byte {}",
                index
            ),
            #[cfg(any(feature = "check", feature = "cb58"))]
            Error::InvalidChecksum {
                checksum,
                expected_checksum,
            } => write!(
                f,
                "invalid checksum, calculated checksum: '{:?}', expected checksum: {:?}",
                checksum, expected_checksum
            ),
            #[cfg(any(feature = "check", feature = "cb58"))]
            Error::InvalidVersion { ver, expected_ver } => write!(
                f,
                "invalid version, payload version: '{:?}', expected version: {:?}",
                ver, expected_ver
            ),
            #[cfg(any(feature = "check", feature = "cb58"))]
            Error::NoChecksum => write!(f, "provided string is too small to contain a checksum"),
        }
    }
}
