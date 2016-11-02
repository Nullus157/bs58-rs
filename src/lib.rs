mod decode;
mod encode;

const DEFAULT_ALPHABET: &'static [u8; 58]
        = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub use decode::FromBase58;
pub use encode::ToBase58;
