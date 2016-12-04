use std::error::Error;
use std::fmt;

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
        }
    }
}
