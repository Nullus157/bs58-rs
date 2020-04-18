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

#[derive(Clone, Copy)]
pub struct Alphabet {
    pub encode: [u8; 58],
    pub decode: [u8; 128],
}

impl Alphabet {
    pub fn new(base: &[u8; 58]) -> Alphabet {
        assert!(base.iter().all(|&c| c < 128));

        let mut encode = [0; 58];
        encode.copy_from_slice(base);

        let mut decode = [0xFF; 128];
        for (i, &c) in base.iter().enumerate() {
            decode[c as usize] = i as u8;
        }

        Alphabet { encode, decode }
    }
}

pub const ALPHABET_BITCOIN: &Alphabet = &Alphabet {
    encode: [
        49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 74, 75, 76, 77, 78, 80,
        81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107,
        109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
    ],
    decode: [
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8,
        255, 255, 255, 255, 255, 255, 255, 9, 10, 11, 12, 13, 14, 15, 16, 255, 17, 18, 19, 20, 21,
        255, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 255, 255, 255, 255, 255, 255, 33, 34, 35,
        36, 37, 38, 39, 40, 41, 42, 43, 255, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
        57, 255, 255, 255, 255, 255,
    ],
};

pub const ALPHABET_MONERO: &Alphabet = &Alphabet {
    encode: [
        49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 74, 75, 76, 77, 78, 80,
        81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107,
        109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
    ],
    decode: [
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8,
        255, 255, 255, 255, 255, 255, 255, 9, 10, 11, 12, 13, 14, 15, 16, 255, 17, 18, 19, 20, 21,
        255, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 255, 255, 255, 255, 255, 255, 33, 34, 35,
        36, 37, 38, 39, 40, 41, 42, 43, 255, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
        57, 255, 255, 255, 255, 255,
    ],
};

pub const ALPHABET_RIPPLE: &Alphabet = &Alphabet {
    encode: [
        114, 112, 115, 104, 110, 97, 102, 51, 57, 119, 66, 85, 68, 78, 69, 71, 72, 74, 75, 76, 77,
        52, 80, 81, 82, 83, 84, 55, 86, 87, 88, 89, 90, 50, 98, 99, 100, 101, 67, 103, 54, 53, 106,
        107, 109, 56, 111, 70, 113, 105, 49, 116, 117, 118, 65, 120, 121, 122,
    ],
    decode: [
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 50, 33, 7, 21, 41, 40, 27,
        45, 8, 255, 255, 255, 255, 255, 255, 255, 54, 10, 38, 12, 14, 47, 15, 16, 255, 17, 18, 19,
        20, 13, 255, 22, 23, 24, 25, 26, 11, 28, 29, 30, 31, 32, 255, 255, 255, 255, 255, 255, 5,
        34, 35, 36, 37, 6, 39, 3, 49, 42, 43, 255, 44, 4, 46, 1, 48, 0, 2, 51, 52, 53, 9, 55, 56,
        57, 255, 255, 255, 255, 255,
    ],
};

pub const ALPHABET_FLICKR: &Alphabet = &Alphabet {
    encode: [
        49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107,
        109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 65, 66, 67, 68, 69,
        70, 71, 72, 74, 75, 76, 77, 78, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
    ],
    decode: [
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8,
        255, 255, 255, 255, 255, 255, 255, 34, 35, 36, 37, 38, 39, 40, 41, 255, 42, 43, 44, 45, 46,
        255, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 255, 255, 255, 255, 255, 255, 9, 10, 11,
        12, 13, 14, 15, 16, 17, 18, 19, 255, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        33, 255, 255, 255, 255, 255,
    ],
};

pub const ALPHABET_DEFAULT: &Alphabet = ALPHABET_BITCOIN;

#[derive(Clone, Copy)]
pub(crate) enum AlphabetCow<'a> {
    Borrowed(&'a Alphabet),
    Owned(Alphabet),
}
