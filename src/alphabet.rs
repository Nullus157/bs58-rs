use core::fmt;

/// Prepared Alphabet for [`EncodeBuilder`](crate::encode::EncodeBuilder) and
/// [`DecodeBuilder`](crate::decode::DecodeBuilder).
#[derive(Clone, Copy)]
pub struct Alphabet {
    pub(crate) encode: [u8; 58],
    pub(crate) decode: [u8; 128],
}

impl fmt::Debug for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(s) = core::str::from_utf8(&self.encode) {
            f.debug_tuple("Alphabet").field(&s).finish()
        } else {
            f.debug_tuple("Alphabet").field(&&self.encode[..]).finish()
        }
    }
}

impl Alphabet {
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

    /// Bitcoin's alphabet as defined in their Base58Check encoding.
    ///
    /// See <https://en.bitcoin.it/wiki/Base58Check_encoding#Base58_symbol_chart>
    pub const BITCOIN: &'static Self =
        &Self::new(b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");

    /// Monero's alphabet as defined in this forum post.
    ///
    /// See <https://forum.getmonero.org/4/academic-and-technical/221/creating-a-standard-for-physical-coins>
    pub const MONERO: &'static Self =
        &Self::new(b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");

    /// Ripple's alphabet as defined in their wiki.
    ///
    /// See <https://wiki.ripple.com/Encodings>
    pub const RIPPLE: &'static Self =
        &Self::new(b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz");

    /// Flickr's alphabet for creating short urls from photo ids.
    ///
    /// See <https://www.flickr.com/groups/api/discuss/72157616713786392/>
    pub const FLICKR: &'static Self =
        &Self::new(b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ");

    /// The default alphabet used if none is given. Currently is the
    /// [`BITCOIN`](Self::BITCOIN) alphabet.
    pub const DEFAULT: &'static Self = Self::BITCOIN;
}

// Force evaluation of the associated constants to make sure they don't error
const _: () = {
    let _ = Alphabet::BITCOIN;
    let _ = Alphabet::MONERO;
    let _ = Alphabet::RIPPLE;
    let _ = Alphabet::FLICKR;
    let _ = Alphabet::DEFAULT;
};
