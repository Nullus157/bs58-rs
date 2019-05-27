#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_results)]
#![warn(variant_size_differences)]

#![allow(unknown_lints)] // For clippy
#![allow(renamed_and_removed_lints)] // clippy namespaced lint compat

#![allow(const_static_lifetime)] // 1.13 compat
#![allow(redundant_field_names)] // 1.13 compat

//! Another [Base58][] codec implementation.
//!
//! Compared to [`base58`][] this is significantly faster at decoding (about
//! 2.4x as fast when decoding 32 bytes), almost the same speed for encoding
//! (about 3% slower when encoding 32 bytes) and doesn't have the 128 byte
//! limitation.
//!
//! Compared to [`rust-base58`][] this is massively faster (over ten times as
//! fast when decoding 32 bytes, almost 40 times as fast when encoding 32
//! bytes) and has no external dependencies.
//!
//! Compared to both this supports a configurable alphabet and user provided
//! buffers for zero-allocation {en,de}coding.
//!
//! [Base58]: https://en.wikipedia.org/wiki/Base58
//! [`base58`]: https://github.com/debris/base58
//! [`rust-base58`]: https://github.com/nham/rust-base58
//!
//! # Optional Features
//!
//! ## `check` (off-by-default)
//!
//! Integrated support for [Base58Check][], this allows automatically
//! calculating the checksum during encoding and verifying during decoding.
//!
//! [Base58Check]: https://en.bitcoin.it/wiki/Base58Check_encoding
//!
//! # Examples
//!
//! ## Basic example
//!
//! ```rust
//! let decoded = bs58::decode("he11owor1d").into_vec().unwrap();
//! let encoded = bs58::encode(decoded).into_string();
//! assert_eq!("he11owor1d", encoded);
//! ```
//!
//! ## Changing the alphabet
//!
//! ```rust
//! let decoded = bs58::decode("he11owor1d")
//!     .with_alphabet(bs58::alphabet::RIPPLE)
//!     .into_vec()
//!     .unwrap();
//! let encoded = bs58::encode(decoded)
//!     .with_alphabet(bs58::alphabet::FLICKR)
//!     .into_string();
//! assert_eq!("4DSSNaN1SC", encoded);
//! ```
//!
//! ## Decoding into an existing buffer
//!
//! ```rust
//! let (mut decoded, mut encoded) = ([0xFF; 8], String::with_capacity(10));
//! bs58::decode("he11owor1d").into(&mut decoded).unwrap();
//! bs58::encode(decoded).into(&mut encoded);
//! assert_eq!("he11owor1d", encoded);
//! ```
//!

#[cfg(feature = "check")]
extern crate sha2;

pub mod alphabet;

pub mod decode;
pub mod encode;
mod error;
mod traits;

const CHECKSUM_LEN: usize = 4;

#[allow(deprecated)]
pub use traits::{ FromBase58, ToBase58 };

/// Setup decoder for the given string using the [default alphabet][].
///
/// [default alphabet]: alphabet/constant.DEFAULT.html
///
/// # Examples
///
/// ## Basic example
///
/// ```rust
/// assert_eq!(
///     vec![0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58],
///     bs58::decode("he11owor1d").into_vec().unwrap());
/// ```
///
/// ## Changing the alphabet
///
/// ```rust
/// assert_eq!(
///     vec![0x60, 0x65, 0xe7, 0x9b, 0xba, 0x2f, 0x78],
///     bs58::decode("he11owor1d")
///         .with_alphabet(bs58::alphabet::RIPPLE)
///         .into_vec().unwrap());
/// ```
///
/// ## Decoding into an existing buffer
///
/// ```rust
/// let mut output = [0xFF; 10];
/// assert_eq!(8, bs58::decode("he11owor1d").into(&mut output).unwrap());
/// assert_eq!(
///     [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58, 0xFF, 0xFF],
///     output);
/// ```
///
/// ## Errors
///
/// ### Invalid Character
///
/// ```rust
/// assert_eq!(
///     bs58::decode::DecodeError::InvalidCharacter { character: 'l', index: 2 },
///     bs58::decode("hello world").into_vec().unwrap_err());
/// ```
///
/// ### Non-ASCII Character
///
/// ```rust
/// assert_eq!(
///     bs58::decode::DecodeError::NonAsciiCharacter { index: 5 },
///     bs58::decode("he11o🇳🇿").into_vec().unwrap_err());
/// ```
///
/// ### Too Small Buffer
///
/// This error can only occur when reading into a provided buffer, when using
/// `.into_vec` a vector large enough is guaranteed to be used.
///
/// ```rust
/// let mut output = [0; 7];
/// assert_eq!(
///     bs58::decode::DecodeError::BufferTooSmall,
///     bs58::decode("he11owor1d").into(&mut output).unwrap_err());
/// ```
pub fn decode<I: AsRef<[u8]>>(input: I) -> decode::DecodeBuilder<'static, I> {
    decode::DecodeBuilder::new(input, alphabet::DEFAULT)
}

/// Setup encoder for the given bytes using the [default alphabet][].
///
/// [default alphabet]: alphabet/constant.DEFAULT.html
///
/// # Examples
///
/// ## Basic example
///
/// ```rust
/// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
/// assert_eq!("he11owor1d", bs58::encode(input).into_string());
/// ```
///
/// ## Changing the alphabet
///
/// ```rust
/// let input = [0x60, 0x65, 0xe7, 0x9b, 0xba, 0x2f, 0x78];
/// assert_eq!(
///     "he11owor1d",
///     bs58::encode(input)
///         .with_alphabet(bs58::alphabet::RIPPLE)
///         .into_string());
/// ```
///
/// ## Encoding into an existing string
///
/// ```rust
/// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
/// let mut output = "goodbye world".to_owned();
/// bs58::encode(input).into(&mut output);
/// assert_eq!("he11owor1d", output);
/// ```
pub fn encode<I: AsRef<[u8]>>(input: I) -> encode::EncodeBuilder<'static, I> {
    encode::EncodeBuilder::new(input, alphabet::DEFAULT)
}

#[cfg(test)]
#[cfg(feature = "check")]
#[macro_use]
extern crate assert_matches;


#[cfg(test)]
const TEST_CASES: &'static [(&'static [u8], &'static str)] = &[
    (&[], ""),
    (&[0x61], "2g"),
    (&[0x62, 0x62, 0x62], "a3gV"),
    (&[0x63, 0x63, 0x63], "aPEr"),
    (&[0x57, 0x2e, 0x47, 0x94], "3EFU7m"),
    (&[0x10, 0xc8, 0x51, 0x1e], "Rt5zm"),
    (&[0x51, 0x6b, 0x6f, 0xcd, 0x0f], "ABnLTmg"),
    (&[0xbf, 0x4f, 0x89, 0x00, 0x1e, 0x67, 0x02, 0x74, 0xdd], "3SEo3LWLoPntC"),
    (&[0xec, 0xac, 0x89, 0xca, 0xd9, 0x39, 0x23, 0xc0, 0x23, 0x21], "EJDM8drfXA6uyA"),
    (&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], "1111111111"),
    (&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], "FPBt6CHo3fovdL"),
    (&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], "NKioeUVktgzXLJ1B3t"),
    (&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], "YcVfxkQb6JRzqk5kF2tNLv"),
    (&[0x73, 0x69, 0x6d, 0x70, 0x6c, 0x79, 0x20, 0x61, 0x20, 0x6c, 0x6f, 0x6e, 0x67, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67], "2cFupjhnEsSn59qHXstmK2ffpLv2"),
    (&[0x00, 0xeb, 0x15, 0x23, 0x1d, 0xfc, 0xeb, 0x60, 0x92, 0x58, 0x86, 0xb6, 0x7d, 0x06, 0x52, 0x99, 0x92, 0x59, 0x15, 0xae, 0xb1, 0x72, 0xc0, 0x66, 0x47], "1NS17iag9jJgTHD1VXjvLCEnZuQ3rJDE9L"),
    (&[0x00, 0x3c, 0x17, 0x6e, 0x65, 0x9b, 0xea, 0x0f, 0x29, 0xa3, 0xe9, 0xbf, 0x78, 0x80, 0xc1, 0x12, 0xb1, 0xb3, 0x1b, 0x4d, 0xc8, 0x26, 0x26, 0x81, 0x87], "16UjcYNBG9GTK4uq2f7yYEbuifqCzoLMGS"),
    (&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], "11111111111111111111111111111111"),
    (&[0x80, 0x11, 0x84, 0xcd, 0x2c, 0xdd, 0x64, 0x0c, 0xa4, 0x2c, 0xfc, 0x3a, 0x09, 0x1c, 0x51, 0xd5, 0x49, 0xb2, 0xf0, 0x16, 0xd4, 0x54, 0xb2, 0x77, 0x40, 0x19, 0xc2, 0xb2, 0xd2, 0xe0, 0x85, 0x29, 0xfd, 0x20, 0x6e, 0xc9, 0x7e], "5Hx15HFGyep2CfPxsJKe2fXJsCVn5DEiyoeGGF6JZjGbTRnqfiD"),
];

#[cfg(test)]
#[cfg(feature = "check")]
const CHECK_TEST_CASES: &'static [(&'static [u8], &'static str)] = &[
    (&[], "3QJmnh"),
    (&[0x31], "6bdbJ1U"),
    (&[0x39], "7VsrQCP"),
    (&[0x2d, 0x31], "PWEu9GGN"),
    (&[0x31, 0x31], "RVnPfpC2"),
    (&[0x31, 0x32, 0x33, 0x34, 0x35, 0x39, 0x38, 0x37, 0x36, 0x30], "K5zqBMZZTzUbAZQgrt4"),
    (&[0x00, 0x9b, 0x41, 0x54, 0xbb, 0xf2, 0x03, 0xe4, 0x13, 0x0c, 0x4b, 0x86, 0x25, 0x93, 0x18, 0xa4, 0x98, 0x75, 0xdd, 0x04, 0x56], "1F9v11cupBVMpz3CrVfCppv9Rw2xEtU1c6"),
    (&[0x53, 0x25, 0xb1, 0xe2, 0x3b, 0x5b, 0x24, 0xf3, 0x47, 0xed, 0x19, 0xde, 0x61, 0x23, 0x8a, 0xf1, 0x4b, 0xc4, 0x71, 0xca, 0xa1, 0xa7, 0x7a, 0xa5, 0x5d, 0xb2, 0xa7, 0xaf, 0x7d, 0xaa, 0x93, 0xaa], "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q"),
];
