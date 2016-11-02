//! Commonly used Base58 alphabets.

/// Bitcoin's alphabet as defined in their Base58Check encoding.
///
/// See https://en.bitcoin.it/wiki/Base58Check_encoding#Base58_symbol_chart.
pub const BITCOIN: &'static [u8; 58]
    = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Monero's alphabet as defined in this forum post.
///
/// See https://forum.getmonero.org/4/academic-and-technical/221/creating-a-standard-for-physical-coins
pub const MONERO: &'static [u8; 58]
    = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Ripple's alphabet as defined in their wiki.
///
/// See https://wiki.ripple.com/Encodings
pub const RIPPLE: &'static [u8; 58]
    = b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz";

/// Flickr's alphabet for creating short urls from photo ids.
///
/// See https://www.flickr.com/groups/api/discuss/72157616713786392/
pub const FLICKR: &'static [u8; 58]
    = b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";

/// The default alphabet used if none is given. Currently is the
/// [`BITCOIN`](constant.BITCOIN.html) alphabet.
pub const DEFAULT: &'static [u8; 58] = BITCOIN;
