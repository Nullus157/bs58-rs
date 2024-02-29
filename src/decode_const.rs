//! Functions for decoding Base58 encoded strings in a const context.

use crate::Alphabet;

/// A builder for setting up the alphabet and output of a base58 decode.
///
/// See the documentation for [`bs58::decode_const`](crate::decode_const()) for
/// a more high level view of how to use this.
#[allow(missing_debug_implementations)]
pub struct DecodeBuilder<'a, 'b> {
    input: &'a [u8],
    alpha: &'b Alphabet,
}

impl<'a, 'b> DecodeBuilder<'a, 'b> {
    /// Setup decoder for the given string using the given alphabet.
    /// Preferably use [`bs58::decode_const`](crate::decode_const()) instead of
    /// this directly.
    pub const fn new(input: &'a [u8], alpha: &'b Alphabet) -> Self {
        Self { input, alpha }
    }

    /// Setup decoder for the given string using default prepared alphabet.
    pub(crate) const fn from_input(input: &'a [u8]) -> Self {
        Self {
            input,
            alpha: Alphabet::DEFAULT,
        }
    }

    /// Change the alphabet that will be used for decoding.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     vec![0x60, 0x65, 0xe7, 0x9b, 0xba, 0x2f, 0x78],
    ///     bs58::decode_const(b"he11owor1d")
    ///         .with_alphabet(bs58::Alphabet::RIPPLE)
    ///         .into_array::<7>());
    /// ```
    pub const fn with_alphabet(self, alpha: &'b Alphabet) -> Self {
        Self { alpha, ..self }
    }

    /// Decode into a new array.
    ///
    /// Returns the decoded array as bytes.
    ///
    /// See the documentation for [`bs58::decode_const`](crate::decode_const())
    /// for an explanation of the errors that may occur.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let output = bs58::decode_const(b"EUYUqQf").into_array::<5>();
    /// assert_eq!(output.len(), 5);
    /// assert_eq!("world", std::str::from_utf8(&output)?);
    /// # Ok::<(), std::str::Utf8Error>(())
    /// ```
    pub const fn into_array<const N: usize>(&self) -> [u8; N] {
        decode_into::<N>(self.input, self.alpha)
    }
}

const fn decode_into<const N: usize>(input: &[u8], alpha: &Alphabet) -> [u8; N] {
    let mut output = [0u8; N];
    let mut index = 0;
    let zero = alpha.encode[0];

    let mut i = 0;
    while i < input.len() {
        let c = input[i];
        assert!(c < 128, "provided string contained a non-ascii character");

        let mut val = alpha.decode[c as usize] as usize;
        assert!(
            val != 0xFF,
            "provided string contained an invalid character"
        );

        let mut j = 0;
        while j < index {
            let byte = output[j];
            val += (byte as usize) * 58;
            output[j] = (val & 0xFF) as u8;
            val >>= 8;
            j += 1;
        }

        while val > 0 {
            output[index] = (val & 0xFF) as u8;
            index += 1;
            val >>= 8
        }
        i += 1;
    }

    let mut i = 0;
    while i < input.len() && input[i] == zero {
        output[index] = 0;
        index += 1;
        i += 1;
    }

    // reverse
    let mut i = 0;
    let n = index / 2;
    while i < n {
        let x = output[i];
        output[i] = output[index - 1 - i];
        output[index - 1 - i] = x;
        i += 1;
    }

    output
}
