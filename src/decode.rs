//! Functions for decoding Base58 encoded strings.

use core::fmt;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::Check;
#[cfg(feature = "check")]
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

    #[cfg(feature = "check")]
    #[cfg_attr(docsrs, doc(cfg(feature = "check")))]
    /// The checksum did not match the payload bytes
    InvalidChecksum {
        ///The given checksum
        checksum: [u8; CHECKSUM_LEN],
        ///The checksum calculated for the payload
        expected_checksum: [u8; CHECKSUM_LEN],
    },

    #[cfg(feature = "check")]
    #[cfg_attr(docsrs, doc(cfg(feature = "check")))]
    /// The version did not match the payload bytes
    InvalidVersion {
        ///The given version
        ver: u8,
        ///The expected version
        expected_ver: u8,
    },

    #[cfg(feature = "check")]
    #[cfg_attr(docsrs, doc(cfg(feature = "check")))]
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
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
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
    #[cfg_attr(docsrs, doc(cfg(feature = "check")))]
    pub fn with_check(self, expected_ver: Option<u8>) -> DecodeBuilder<'a, I> {
        let check = Check::Enabled(expected_ver);
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
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    pub fn into_vec(self) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        self.into(&mut output)?;
        Ok(output)
    }

    /// Decode into a new vector of bytes.
    ///
    /// This method decodes multiple bytes simultaneously and allocates more memory than strictly
    /// necessary (by a constant number of bytes). Simultaneously, this method does not obey the
    /// `Check::Enabled` flag.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    pub fn into_vec_unsafe(self) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        output.resize((self.input.as_ref().len() + 3) / 4 * 4, 0);

        let len = decode_into_limbs(self.input.as_ref(), &mut output, self.alpha)?;
        output.truncate(len);
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
                decode_check_into(self.input.as_ref(), output, &self.alpha, expected_ver)
            }),
        }
    }
}

fn decode_into(input: &[u8], output: &mut [u8], alpha: &Alphabet) -> Result<usize> {
    let mut index = 0;
    let zero = alpha.encode[0];

    for (i, c) in input.iter().enumerate() {
        if *c > 127 {
            return Err(Error::NonAsciiCharacter { index: i });
        }

        let mut val = alpha.decode[*c as usize] as usize;
        if val == 0xFF {
            return Err(Error::InvalidCharacter {
                character: *c as char,
                index: i,
            });
        }

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

    for _ in input.iter().take_while(|c| **c == zero) {
        let byte = output.get_mut(index).ok_or(Error::BufferTooSmall)?;
        *byte = 0;
        index += 1;
    }

    output[..index].reverse();
    Ok(index)
}

fn decode_into_limbs(input: &[u8], output: &mut [u8], alpha: &Alphabet) -> Result<usize> {
    let input_bytes_per_limb = 5; // 58**5 < 2**32

    let decode_input_byte = |(i, c): (usize, &u8)| -> Result<usize> {
        if *c > 127 {
            return Err(Error::NonAsciiCharacter { index: i });
        }

        let val = alpha.decode[*c as usize] as usize;
        if val == 0xFF {
            return Err(Error::InvalidCharacter {
                character: *c as char,
                index: i,
            });
        }
        Ok(val)
    };

    let mut index = 0;
    let mut input_iter = input.iter().enumerate();
    let next_limb_multiplier = 58 * 58 * 58 * 58 * 58;

    let (prefix, output_as_limbs, _) = unsafe { output.align_to_mut::<u32>() };
    let prefix_len = prefix.len();

    while input_iter.len() >= input_bytes_per_limb {
        let input_byte0 = decode_input_byte(input_iter.next().unwrap())?;
        let input_byte1 = decode_input_byte(input_iter.next().unwrap())?;
        let input_byte2 = decode_input_byte(input_iter.next().unwrap())?;
        let input_byte3 = decode_input_byte(input_iter.next().unwrap())?;
        let input_byte4 = decode_input_byte(input_iter.next().unwrap())?;

        let mut next_limb
                 = input_byte0 * 58 * 58 * 58 * 58
                 + input_byte1 * 58 * 58 * 58
                 + input_byte2 * 58 * 58
                 + input_byte3 * 58
                 + input_byte4
                 ;

        for limb in &mut output_as_limbs[..index] {
            next_limb += (*limb as usize) * next_limb_multiplier;
            *limb = (next_limb & 0xFFFFFFFF) as u32;
            next_limb >>= 32;
        }

        while next_limb > 0 {
            let limb = output_as_limbs.get_mut(index).ok_or(Error::BufferTooSmall)?;
            *limb = (next_limb & 0xFFFFFFFF) as u32;
            index += 1;
            next_limb >>= 32;
        }
    }

    if input_iter.len() > 0 {
        let mut next_limb = 0;
        let mut last_limb_multiplier = 1;
        for input_byte in input_iter {
            next_limb = next_limb * 58 + decode_input_byte(input_byte)?;
            last_limb_multiplier = last_limb_multiplier * 58;
        }

        for limb in &mut output_as_limbs[..index] {
            next_limb += (*limb as usize) * last_limb_multiplier;
            *limb = (next_limb & 0xFFFFFFFF) as u32;
            next_limb >>= 32;
        }

        while next_limb > 0 {
            let limb = output_as_limbs.get_mut(index).ok_or(Error::BufferTooSmall)?;
            *limb = (next_limb & 0xFFFFFFFF) as u32;
            index += 1;
            next_limb >>= 32;
        }
    }

    // rescale for the remainder
    index = index * 4;
    {
    let output = &mut output[prefix_len..];
    while index > 0 && output[index - 1] == 0 {
        index -= 1;
    }

    let zero = alpha.encode[0];
    for _ in input.iter().take_while(|c| **c == zero) {
        let byte = output.get_mut(index).ok_or(Error::BufferTooSmall)?;
        *byte = 0;
        index += 1;
    }

    output[..index].reverse();
    }

    if prefix_len > 0 {
        output.copy_within(prefix_len..prefix_len + index, 0);
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
    let second_hash = Sha256::digest(&first_hash);
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

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
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
            #[cfg(feature = "check")]
            Error::InvalidChecksum {
                checksum,
                expected_checksum,
            } => write!(
                f,
                "invalid checksum, calculated checksum: '{:?}', expected checksum: {:?}",
                checksum, expected_checksum
            ),
            #[cfg(feature = "check")]
            Error::InvalidVersion { ver, expected_ver } => write!(
                f,
                "invalid version, payload version: '{:?}', expected version: {:?}",
                ver, expected_ver
            ),
            #[cfg(feature = "check")]
            Error::NoChecksum => write!(f, "provided string is too small to contain a checksum"),
        }
    }
}
