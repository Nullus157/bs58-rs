//! Commonly used Base58 alphabets.

/// Bitcoin's alphabet as defined in their Base58Check encoding.
///
/// See https://en.bitcoin.it/wiki/Base58Check_encoding#Base58_symbol_chart.
pub const BITCOIN: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Monero's alphabet as defined in this forum post.
///
/// See https://forum.getmonero.org/4/academic-and-technical/221/creating-a-standard-for-physical-coins
pub const MONERO: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Ripple's alphabet as defined in their wiki.
///
/// See https://wiki.ripple.com/Encodings
pub const RIPPLE: &[u8; 58] = b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz";

/// Flickr's alphabet for creating short urls from photo ids.
///
/// See https://www.flickr.com/groups/api/discuss/72157616713786392/
pub const FLICKR: &[u8; 58] = b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";

/// The default alphabet used if none is given. Currently is the
/// [`BITCOIN`](constant.BITCOIN.html) alphabet.
pub const DEFAULT: &[u8; 58] = BITCOIN;

/// Prepared Alphabet for [`EncodeBuilder`](crate::encode::EncodeBuilder) and
/// [`DecodeBuilder`](crate::decode::DecodeBuilder).
#[derive(Clone, Copy)]
#[allow(missing_debug_implementations)]
pub struct Alphabet {
    pub(crate) encode: [u8; 58],
    pub(crate) decode: [u8; 128],
}

impl Alphabet {
    /// Bitcoin's prepared alphabet.
    pub const BITCOIN: &'static Self = &Self::new(BITCOIN);
    /// Monero's prepared alphabet.
    pub const MONERO: &'static Self = &Self::new(MONERO);
    /// Ripple's prepared alphabet.
    pub const RIPPLE: &'static Self = &Self::new(RIPPLE);
    /// Flickr's prepared alphabet.
    pub const FLICKR: &'static Self = &Self::new(FLICKR);
    /// The default prepared alphabet used if none is given. Currently is the
    /// [`Alphabet::Bitcoin`](Alphabet::BITCOIN) alphabet.
    pub const DEFAULT: &'static Self = Self::BITCOIN;

    /// Create prepared alphabet.
    pub const fn new(base: &[u8; 58]) -> Alphabet {
        let mut encode = [0x00; 58];
        let mut decode = [0xFF; 128];

        let mut i = 0;
        while i < encode.len() {
            encode[i] = base[i];
            decode[base[i] as usize] = i as u8;
            i += 1;
        }

        Alphabet { encode, decode }
    }
}

/// `std::borrow::Cow` alternative.
#[allow(variant_size_differences)]
pub(crate) enum AlphabetCow<'a> {
    Borrowed(&'a Alphabet),
    Owned(Alphabet),
}
