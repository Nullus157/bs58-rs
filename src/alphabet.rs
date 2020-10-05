use core::fmt;

/// Prepared Alphabet for
/// [`EncodeBuilder::with_alphabet`](crate::encode::EncodeBuilder::with_alphabet) and
/// [`DecodeBuilder::with_alphabet`](crate::decode::DecodeBuilder::with_alphabet).
#[derive(Clone, Copy)]
pub struct Alphabet {
    pub(crate) encode: [u8; 58],
    pub(crate) decode: [u8; 128],
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum Error {
    DuplicateCharacter {
        character: char,
        first: usize,
        second: usize,
    },
    NonAsciiCharacter {
        index: usize,
    },
}

impl Alphabet {
    /// Create prepared alphabet, checks that the alphabet is pure ASCII and that there are no
    /// duplicate characters, which would result in inconsistent encoding/decoding
    ///
    /// ```rust
    /// let alpha = bs58::Alphabet::new(
    ///     b" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXY"
    /// )?;
    ///
    /// let decoded = bs58::decode("he11owor1d")
    ///     .with_alphabet(&bs58::Alphabet::RIPPLE)
    ///     .into_vec()?;
    /// let encoded = bs58::encode(decoded)
    ///     .with_alphabet(&alpha)
    ///     .into_string();
    ///
    /// assert_eq!("#ERRN)N RD", encoded);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub const fn new(base: &[u8; 58]) -> Result<Self, Error> {
        let mut encode = [0x00; 58];
        let mut decode = [0xFF; 128];

        let mut i = 0;
        while i < encode.len() {
            if base[i] >= 128 {
                return Err(Error::NonAsciiCharacter { index: i });
            }
            if decode[base[i] as usize] != 0xFF {
                return Err(Error::DuplicateCharacter {
                    character: base[i] as char,
                    first: decode[base[i] as usize] as usize,
                    second: i,
                });
            }
            encode[i] = base[i];
            decode[base[i] as usize] = i as u8;
            i += 1;
        }

        Ok(Self { encode, decode })
    }

    /// Same as [`Self::new`], but gives a panic instead of an [`Err`] on bad input.
    ///
    /// Intended to support usage in `const` context until [`Result::unwrap`] is able to be called.
    ///
    /// ```rust
    /// const ALPHA: bs58::Alphabet = bs58::Alphabet::new_unwrap(
    ///     b" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXY"
    /// );
    ///
    /// let decoded = bs58::decode("he11owor1d")
    ///     .with_alphabet(&bs58::Alphabet::RIPPLE)
    ///     .into_vec()?;
    /// let encoded = bs58::encode(decoded)
    ///     .with_alphabet(&ALPHA)
    ///     .into_string();
    ///
    /// assert_eq!("#ERRN)N RD", encoded);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub const fn new_unwrap(base: &[u8; 58]) -> Self {
        let result = Self::new(base);
        let _ = [0][result.is_err() as usize];
        match result {
            Ok(alphabet) => alphabet,
            // unreachable, for type checking
            Err(_) => Self {
                encode: [0; 58],
                decode: [0; 128],
            },
        }
    }

    /// Bitcoin's alphabet as defined in their Base58Check encoding.
    ///
    /// See <https://en.bitcoin.it/wiki/Base58Check_encoding#Base58_symbol_chart>
    pub const BITCOIN: Self =
        Self::new_unwrap(b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");

    /// Monero's alphabet as defined in this forum post.
    ///
    /// See <https://forum.getmonero.org/4/academic-and-technical/221/creating-a-standard-for-physical-coins>
    pub const MONERO: Self =
        Self::new_unwrap(b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");

    /// Ripple's alphabet as defined in their wiki.
    ///
    /// See <https://wiki.ripple.com/Encodings>
    pub const RIPPLE: Self =
        Self::new_unwrap(b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz");

    /// Flickr's alphabet for creating short urls from photo ids.
    ///
    /// See <https://www.flickr.com/groups/api/discuss/72157616713786392/>
    pub const FLICKR: Self =
        Self::new_unwrap(b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ");

    /// The default alphabet used if none is given. Currently is the
    /// [`BITCOIN`](Self::BITCOIN) alphabet.
    pub const DEFAULT: Self = Self::BITCOIN;
}

impl fmt::Debug for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(s) = core::str::from_utf8(&self.encode) {
            f.debug_tuple("Alphabet").field(&s).finish()
        } else {
            unreachable!()
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::DuplicateCharacter {
                character,
                first,
                second,
            } => write!(
                f,
                "alphabet contained a duplicate character `{}` at indexes {} and {}",
                character, first, second,
            ),
            Error::NonAsciiCharacter { index } => {
                write!(f, "alphabet contained a non-ascii character at {}", index)
            }
        }
    }
}

// Force evaluation of the associated constants to make sure they don't error
const _: () = {
    let _ = Alphabet::BITCOIN;
    let _ = Alphabet::MONERO;
    let _ = Alphabet::RIPPLE;
    let _ = Alphabet::FLICKR;
    let _ = Alphabet::DEFAULT;
};
