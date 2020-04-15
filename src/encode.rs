//! Functions for encoding into Base58 encoded strings.

use core::fmt;

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

#[cfg(feature = "check")]
use crate::CHECKSUM_LEN;

/// A builder for setting up the alphabet and output of a base58 encode.
#[allow(missing_debug_implementations)]
pub struct EncodeBuilder<'a, I: AsRef<[u8]>> {
    input: I,
    alpha: &'a [u8; 58],
    check: bool,
}

/// A specialized [`Result`](core::result::Result) type for [`bs58::encode`](module@crate::encode)
pub type Result<T> = core::result::Result<T, Error>;

/// Errors that could occur when encoding a Base58 encoded string.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// The output buffer was too small to contain the entire input.
    BufferTooSmall,

    #[doc(hidden)]
    __NonExhaustive,
}

/// Represents a buffer that can be encoded into. See [`EncodeBuilder::into`] and the provided
/// implementations for more details.
pub trait EncodeTarget {
    /// Encodes into this buffer, provides the maximum length for implementations that wish to
    /// preallocate space, along with a function that will encode ASCII bytes into the buffer and
    /// return the length written to it.
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize>;
}

impl<T: EncodeTarget + ?Sized> EncodeTarget for &mut T {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        T::encode_with(self, max_len, f)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
impl EncodeTarget for Vec<u8> {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        self.resize(max_len, 0);
        let len = f(&mut *self)?;
        self.truncate(len);
        Ok(len)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
impl EncodeTarget for String {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        let mut output = core::mem::replace(self, String::new()).into_bytes();
        let len = output.encode_with(max_len, f)?;
        *self = String::from_utf8(output).unwrap();
        Ok(len)
    }
}

impl EncodeTarget for [u8] {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        let _ = max_len;
        f(&mut *self)
    }
}

impl EncodeTarget for str {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        struct Guard<'a>(&'a mut [u8]);

        impl Drop for Guard<'_> {
            fn drop(&mut self) {
                let mut index = 0;
                loop {
                    match core::str::from_utf8(&self.0[index..]) {
                        Ok(_) => return,
                        Err(e) => {
                            index += e.valid_up_to();
                            if let Some(len) = e.error_len() {
                                for i in &mut self.0[index..index + len] {
                                    *i = 0;
                                }
                                index += len;
                            } else {
                                for i in &mut self.0[index..] {
                                    *i = 0;
                                }
                                index += self.0[index..].len();
                            }
                        }
                    }
                }
            }
        }

        let _ = max_len;

        let guard = Guard(unsafe { self.as_bytes_mut() });
        f(&mut *guard.0)
    }
}

impl<'a, I: AsRef<[u8]>> EncodeBuilder<'a, I> {
    /// Setup encoder for the given string using the given alphabet.
    /// Preferably use [`bs58::encode`](../fn.encode.html) instead of this
    /// directly.
    pub fn new(input: I, alpha: &'a [u8; 58]) -> EncodeBuilder<'a, I> {
        EncodeBuilder {
            input,
            alpha,
            check: false,
        }
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
    pub fn with_alphabet(self, alpha: &[u8; 58]) -> EncodeBuilder<'_, I> {
        EncodeBuilder {
            input: self.input,
            alpha,
            check: self.check,
        }
    }

    /// Include checksum calculated using the [Base58Check][] algorithm when
    /// encoding.
    ///
    /// [Base58Check]: https://en.bitcoin.it/wiki/Base58Check_encoding
    ///
    /// # Examples
    ///
    /// ```rust
    /// let input = [0x60, 0x65, 0xe7, 0x9b, 0xba, 0x2f, 0x78];
    /// assert_eq!(
    ///     "QuT57JNzzWTu7mW",
    ///     bs58::encode(input)
    ///         .with_check()
    ///         .into_string());
    /// ```
    #[cfg(feature = "check")]
    #[cfg_attr(docsrs, doc(cfg(feature = "check")))]
    pub fn with_check(mut self) -> EncodeBuilder<'a, I> {
        self.check = true;
        self
    }

    /// Encode into a new owned string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
    /// assert_eq!("he11owor1d", bs58::encode(input).into_string());
    /// ```
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    pub fn into_string(self) -> String {
        let mut output = String::new();
        self.into(&mut output).unwrap();
        output
    }

    /// Encode into a new owned vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
    /// assert_eq!(b"he11owor1d", &*bs58::encode(input).into_vec());
    /// ```
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    pub fn into_vec(self) -> Vec<u8> {
        let mut output = Vec::new();
        self.into(&mut output).unwrap();
        output
    }

    /// Encode into the given buffer.
    ///
    /// Returns the length written into the buffer.
    ///
    /// If the buffer is resizeable it will be reallocated to fit the encoded data and truncated to
    /// size.
    ///
    /// If the buffer is not resizeable bytes after the final character will be left alone, except
    /// up to 3 null bytes may be written to an `&mut str` to overwrite remaining characters of a
    /// partially overwritten multi-byte character.
    ///
    /// See the documentation for [`bs58::encode`](../fn.encode.html) for an
    /// explanation of the errors that may occur.
    ///
    /// # Examples
    ///
    /// ## `Vec<u8>`
    ///
    /// ```rust
    /// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
    /// let mut output = "goodbye world".to_owned().into_bytes();
    /// bs58::encode(input).into(&mut output);
    /// assert_eq!(b"he11owor1d", &*output);
    /// ```
    ///
    /// ## `&mut [u8]`
    ///
    /// ```rust
    /// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
    /// let mut output = Vec::from("goodbye world");
    /// bs58::encode(input).into(&mut output[..]);
    /// assert_eq!(b"he11owor1drld", &*output);
    /// ```
    ///
    /// ## `String`
    ///
    /// ```rust
    /// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
    /// let mut output = "goodbye world".to_owned();
    /// bs58::encode(input).into(&mut output);
    /// assert_eq!("he11owor1d", output);
    /// ```
    ///
    /// ## `&mut str`
    ///
    /// ```rust
    /// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
    /// let mut output = "goodbye world".to_owned();
    /// bs58::encode(input).into(output.as_mut_str());
    /// assert_eq!("he11owor1drld", output);
    /// ```
    ///
    /// ### Clearing partially overwritten characters
    ///
    /// ```rust
    /// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
    /// let mut output = "goodbye w®ld".to_owned();
    /// bs58::encode(input).into(output.as_mut_str());
    /// assert_eq!("he11owor1d\0ld", output);
    /// ```
    pub fn into(self, mut output: impl EncodeTarget) -> Result<usize> {
        #[cfg(feature = "check")]
        {
            if self.check {
                let max_encoded_len = ((self.input.as_ref().len() + CHECKSUM_LEN) / 5 + 1) * 8;
                return output.encode_with(max_encoded_len, |output| {
                    encode_check_into(self.input.as_ref(), output, self.alpha)
                });
            }
        }

        let max_encoded_len = (self.input.as_ref().len() / 5 + 1) * 8;
        output.encode_with(max_encoded_len, |output| {
            encode_into(self.input.as_ref(), output, self.alpha)
        })
    }
}

fn encode_into<'a, I>(input: I, output: &mut [u8], alpha: &[u8; 58]) -> Result<usize>
where
    I: Clone + IntoIterator<Item = &'a u8>,
{
    assert!(alpha.iter().all(|&c| c < 128));

    let mut index = 0;
    for &val in input.clone() {
        let mut carry = val as usize;
        for byte in &mut output[..index] {
            carry += (*byte as usize) << 8;
            *byte = (carry % 58) as u8;
            carry /= 58;
        }
        while carry > 0 {
            if index == output.len() {
                return Err(Error::BufferTooSmall);
            }
            output[index] = (carry % 58) as u8;
            index += 1;
            carry /= 58;
        }
    }

    for &val in input {
        if val == 0 {
            if index == output.len() {
                return Err(Error::BufferTooSmall);
            }
            output[index] = 0;
            index += 1;
        } else {
            break;
        }
    }

    for val in &mut output[..index] {
        *val = alpha[*val as usize];
    }

    output[..index].reverse();
    Ok(index)
}

#[cfg(feature = "check")]
fn encode_check_into(input: &[u8], output: &mut [u8], alpha: &[u8; 58]) -> Result<usize> {
    use sha2::{Digest, Sha256};

    let first_hash = Sha256::digest(input);
    let second_hash = Sha256::digest(&first_hash);

    let checksum = &second_hash[0..CHECKSUM_LEN];

    encode_into(input.iter().chain(checksum.iter()), output, alpha)
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BufferTooSmall => write!(
                f,
                "buffer provided to encode base58 string into was too small"
            ),
            Error::__NonExhaustive => unreachable!(),
        }
    }
}
