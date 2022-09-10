mod cases;

const FILLER: [u8; 512] = [b'~'; 512];

#[test]
fn test_encode() {
    for &(val, s) in cases::TEST_CASES.iter() {
        assert_eq!(s, String::from(bs58::encode(val)));

        assert_eq!(s.as_bytes(), Vec::from(bs58::encode(val)));

        {
            let mut bytes = FILLER;
            assert_eq!(Ok(s.len()), bs58::encode(val).onto(&mut bytes[..]));
            assert_eq!(s.as_bytes(), &bytes[..s.len()]);
            assert_eq!(&FILLER[s.len()..], &bytes[s.len()..]);
        }

        {
            let mut bytes = FILLER;
            if !s.is_empty() {
                bytes[(s.len() - 1)..=s.len()].copy_from_slice("Ę".as_bytes());
            }
            let string = core::str::from_utf8_mut(&mut bytes[..]).unwrap();
            assert_eq!(Ok(s.len()), bs58::encode(val).onto(string));
            assert_eq!(s.as_bytes(), &bytes[..s.len()]);
            if !s.is_empty() {
                assert_eq!(0, bytes[s.len()]);
            }
            assert_eq!(&FILLER[(s.len() + 1)..], &bytes[(s.len() + 1)..]);
        }
    }
}

#[test]
#[cfg(feature = "check")]
fn test_encode_check() {
    for &(val, s) in cases::CHECK_TEST_CASES.iter() {
        assert_eq!(s, String::from(bs58::encode(val).with_check()));

        assert_eq!(s.as_bytes(), Vec::from(bs58::encode(val).with_check()));

        {
            let mut bytes = FILLER;
            assert_eq!(
                Ok(s.len()),
                bs58::encode(val).with_check().onto(&mut bytes[..])
            );
            assert_eq!(s.as_bytes(), &bytes[..s.len()]);
            assert_eq!(&FILLER[s.len()..], &bytes[s.len()..]);

            if !val.is_empty() {
                assert_eq!(
                    Ok(s.len()),
                    bs58::encode(&val[1..])
                        .with_check_version(val[0])
                        .onto(&mut bytes[..])
                );
                assert_eq!(s.as_bytes(), &bytes[..s.len()]);
                assert_eq!(&FILLER[s.len()..], &bytes[s.len()..]);
            }
        }

        {
            let mut bytes = FILLER;
            if !s.is_empty() {
                bytes[(s.len() - 1)..=s.len()].copy_from_slice("Ę".as_bytes());
            }
            let string = core::str::from_utf8_mut(&mut bytes[..]).unwrap();
            assert_eq!(Ok(s.len()), bs58::encode(val).with_check().onto(string));
            assert_eq!(s.as_bytes(), &bytes[..s.len()]);
            if !s.is_empty() {
                assert_eq!(0, bytes[s.len()]);
            }
            assert_eq!(&FILLER[(s.len() + 1)..], &bytes[(s.len() + 1)..]);
        }
    }
}

#[test]
fn append() {
    let mut buf = "hello world".to_string();
    bs58::encode(&[92]).onto(&mut buf).unwrap();
    assert_eq!("hello world2b", buf.as_str());
}

/// Verify that encode_into doesn’t try to write over provided buffer.
#[test]
fn test_buffer_too_small() {
    let mut output = [0u8; 256];
    for &(val, s) in cases::TEST_CASES.iter() {
        let expected_len = s.len();
        if expected_len > 0 {
            let res = bs58::encode(val).onto(&mut output[..(expected_len - 1)]);
            assert_eq!(Err(bs58::encode::Error::BufferTooSmall), res);
        }
        let res = bs58::encode(val).onto(&mut output[..expected_len]);
        assert_eq!(Ok(expected_len), res);
    }
}

/// Verify that encode_into doesn’t try to write over provided buffer.
#[test]
#[cfg(feature = "check")]
fn test_buffer_too_small_check() {
    let mut output = [0u8; 256];
    for &(val, s) in cases::CHECK_TEST_CASES.iter() {
        let expected_len = s.len();
        if expected_len > 0 {
            let res = bs58::encode(val)
                .with_check()
                .onto(&mut output[..(expected_len - 1)]);
            assert_eq!(Err(bs58::encode::Error::BufferTooSmall), res);
        }
        let res = bs58::encode(val)
            .with_check()
            .onto(&mut output[..expected_len]);
        assert_eq!(Ok(expected_len), res);
    }
}

/// Stress test encoding by trying to encode increasingly long buffers.
#[test]
fn encode_stress_test() {
    let input = b"\xff".repeat(512);
    for len in 0..=input.len() {
        let _ = String::from(bs58::encode(&input[..len]));
        #[cfg(feature = "check")]
        let _ = String::from(bs58::encode(&input[..len]).with_check());
        #[cfg(feature = "check")]
        let _ = String::from(bs58::encode(&input[..len]).with_check_version(255));
    }
}
