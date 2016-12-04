use { encode, decode, DecodeError };

/// Errors that could occur when decoding a Base58 encoded string.
#[deprecated(since = "0.2.0")]
pub type FromBase58Error = DecodeError;

/// A trait for Base58 encoding bytes to an owned string.
#[allow(deprecated)]
#[deprecated(
    since = "0.2.0",
    note = "Use `bs58::encode` and associated functions instead"
)]
pub trait ToBase58 {
    /// Base58 encode `self` to an owned string using the [default alphabet][].
    ///
    /// [default alphabet]: alphabet/constant.DEFAULT.html
    fn to_base58(&self) -> String;

    /// Base58 encode `self` to an owned string using the given alphabet.
    fn to_base58_with_alphabet(&self, alpha: &[u8; 58]) -> String;
}

/// A trait for decoding Base58 encoded values to a vector of bytes.
#[allow(deprecated)]
#[deprecated(
    since = "0.2.0",
    note = "Use `bs58::decode` and associated functions instead"
)]
pub trait FromBase58 {
    /// Decode `self` to a vector of bytes using the [default alphabet][].
    ///
    /// [default alphabet]: alphabet/constant.DEFAULT.html
    fn from_base58(&self) -> Result<Vec<u8>, FromBase58Error>;

    /// Decode `self` to a vector of bytes using the given alphabet.
    fn from_base58_with_alphabet(&self, alpha: &[u8; 58]) -> Result<Vec<u8>, FromBase58Error>;
}

#[allow(deprecated)]
impl FromBase58 for str {
    #[allow(deprecated)]
    fn from_base58(&self) -> Result<Vec<u8>, FromBase58Error> {
        decode(self).into_vec().map_err(FromBase58Error::from)
    }

    #[allow(deprecated)]
    fn from_base58_with_alphabet(&self, alpha: &[u8; 58]) -> Result<Vec<u8>, FromBase58Error> {
        decode(self).with_alphabet(alpha).into_vec().map_err(FromBase58Error::from)
    }
}

#[allow(deprecated)]
impl ToBase58 for [u8] {
    #[allow(deprecated)]
    fn to_base58(&self) -> String {
        encode(self).into_string()
    }

    #[allow(deprecated)]
    fn to_base58_with_alphabet(&self, alpha: &[u8; 58]) -> String {
        encode(self).with_alphabet(alpha).into_string()
    }
}

// Subset of test cases from https://github.com/cryptocoinjs/base-x/blob/master/test/fixtures.json
#[cfg(test)]
mod tests {
    #[allow(deprecated)]
    use { FromBase58, ToBase58 };

    #[test]
    #[allow(deprecated)]
    fn test_to_base58() {
        for &(val, s) in super::super::TEST_CASES.iter() {
            assert_eq!(s, val.to_base58());
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_from_base58() {
        for &(val, s) in super::super::TEST_CASES.iter() {
            assert_eq!(val.to_vec(), s.from_base58().unwrap());
        }
    }
}
