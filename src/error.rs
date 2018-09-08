use std::error::Error;
use std::fmt;

#[cfg(feature = "check")]
use CHECKSUM_LEN;

/// Errors that could occur when decoding a Base58 encoded string.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DecodeError {
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

impl Error for DecodeError {
    fn description(&self) -> &str {
        match *self {
            DecodeError::BufferTooSmall =>
                "buffer provided to decode base58 encoded string into was too small",
            DecodeError::NonAsciiCharacter { .. } =>
                "base58 encoded string contained a non-ascii character",
            DecodeError::InvalidCharacter { .. } =>
                "base58 encoded string contained an invalid character",
            #[cfg(feature = "check")]
            DecodeError::InvalidChecksum { .. } =>
                "base58 decode check did not match payload checksum with expected checksum",
            #[cfg(feature = "check")]
            DecodeError::InvalidVersion { .. } =>
                "base58 decode check did not match payload version with expected version",
            #[cfg(feature = "check")]
            DecodeError::NoChecksum { .. } =>
                "base58 encoded string does not contained enough bytes to have a checksum",
            DecodeError::__NonExhaustive => unreachable!(),
        }
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecodeError::BufferTooSmall => write!(f,
                "buffer provided to decode base58 encoded string into was too small"),
            DecodeError::InvalidCharacter { character, index } => write!(f,
                "provided string contained invalid character {:?} at byte {}",
                character,
                index),
            DecodeError::NonAsciiCharacter { index } => write!(f,
                "provided string contained non-ascii character starting at byte {}",
                index),
            #[cfg(feature = "check")]
            DecodeError::InvalidChecksum { checksum, expected_checksum } => write!(f,
                "invalid checksum, calculated checksum: '{:?}', expected checksum: {:?}",
                checksum,
                expected_checksum),
            #[cfg(feature = "check")]
            DecodeError::InvalidVersion { ver, expected_ver } => write!(f,
                "invalid version, payload version: '{:?}', expected version: {:?}",
                ver,
                expected_ver),
            #[cfg(feature = "check")]
            DecodeError::NoChecksum => write!(f,
                "provided string is too small to contain a checksum"),
            DecodeError::__NonExhaustive => unreachable!(),
        }
    }
}
