extern crate bs58;

mod cases;

#[allow(deprecated)]
use bs58::{FromBase58, ToBase58};

#[test]
#[allow(deprecated)]
fn test_to_base58() {
    for &(val, s) in cases::TEST_CASES.iter() {
        assert_eq!(s, val.to_base58());
    }
}

#[test]
#[allow(deprecated)]
fn test_from_base58() {
    for &(val, s) in cases::TEST_CASES.iter() {
        assert_eq!(val.to_vec(), s.from_base58().unwrap());
    }
}
