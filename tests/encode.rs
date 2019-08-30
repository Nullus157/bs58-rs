mod cases;

#[test]
fn test_encode() {
    for &(val, s) in cases::TEST_CASES.iter() {
        assert_eq!(s, bs58::encode(val).into_string())
    }
}

#[test]
#[cfg(feature = "check")]
fn test_encode_check() {
    for &(val, s) in cases::CHECK_TEST_CASES.iter() {
        assert_eq!(s, bs58::encode(val).with_check().into_string())
    }
}
