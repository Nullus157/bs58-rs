#![feature(test)]

extern crate test;

macro_rules! case {
    ($name:ident ($decoded_length:expr) $decoded:expr => $encoded:expr) => {
        mod $name {
            mod decode {
                #[bench]
                fn base58(b: &mut ::test::Bencher) {
                    use base58::FromBase58;
                    let temp = $encoded;
                    b.iter(|| temp.from_base58().unwrap());
                }
                #[bench]
                fn rust_base58(b: &mut ::test::Bencher) {
                    use rust_base58::FromBase58;
                    let temp = $encoded;
                    b.iter(|| temp.from_base58().unwrap());
                }
                #[bench]
                fn bs58(b: &mut ::test::Bencher) {
                    b.iter(|| ::bs58::decode($encoded).into_vec().unwrap());
                }
                #[bench]
                fn bs58_noalloc_slice(b: &mut ::test::Bencher) {
                    let mut output = [0; $decoded_length];
                    b.iter(|| ::bs58::decode($encoded).into(&mut output[..]).unwrap());
                }
                #[bench]
                fn bs58_noalloc_array(b: &mut ::test::Bencher) {
                    let mut output = [0; $decoded_length];
                    b.iter(|| ::bs58::decode($encoded).into(&mut output).unwrap());
                }
            }

            mod encode {
                #[bench]
                fn base58(b: &mut ::test::Bencher) {
                    use base58::ToBase58;
                    let temp = $decoded;
                    b.iter(|| temp.to_base58());
                }
                #[bench]
                fn rust_base58(b: &mut ::test::Bencher) {
                    use rust_base58::ToBase58;
                    let temp = $decoded;
                    b.iter(|| temp.to_base58());
                }
                #[bench]
                fn bs58(b: &mut ::test::Bencher) {
                    b.iter(|| ::bs58::encode($decoded).into_string());
                }
                #[bench]
                fn bs58_noalloc(b: &mut ::test::Bencher) {
                    let mut output = String::with_capacity($encoded.len());
                    b.iter(|| ::bs58::encode($decoded).into(&mut output));
                }
            }
        }
    };

    ($name:ident {$decoded_length:expr} $decoded:expr => $encoded:expr) => {
        mod $name {
            mod decode {
                // base58 can't handle more than 32 bytes
                #[bench]
                fn rust_base58(b: &mut ::test::Bencher) {
                    use rust_base58::FromBase58;
                    let temp = $encoded;
                    b.iter(|| temp.from_base58().unwrap());
                }
                #[bench]
                fn bs58(b: &mut ::test::Bencher) {
                    b.iter(|| ::bs58::decode($encoded).into_vec().unwrap());
                }
                #[bench]
                fn bs58_noalloc_slice(b: &mut ::test::Bencher) {
                    let mut output = [0; $decoded_length];
                    b.iter(|| ::bs58::decode($encoded).into(&mut output[..]).unwrap());
                }
                // TODO: bs58_noalloc_array is not possible because of limited array lengths in
                // trait impls
            }

            mod encode {
                #[bench]
                fn base58(b: &mut ::test::Bencher) {
                    use base58::ToBase58;
                    let temp = $decoded;
                    b.iter(|| temp.to_base58());
                }
                #[bench]
                fn rust_base58(b: &mut ::test::Bencher) {
                    use rust_base58::ToBase58;
                    let temp = $decoded;
                    b.iter(|| temp.to_base58());
                }
                #[bench]
                fn bs58(b: &mut ::test::Bencher) {
                    b.iter(|| ::bs58::encode($decoded).into_string());
                }
                #[bench]
                fn bs58_noalloc(b: &mut ::test::Bencher) {
                    let mut output = String::with_capacity($encoded.len());
                    b.iter(|| ::bs58::encode($decoded).into(&mut output));
                }
            }
        }
    };
}

case! { a_empty (0) vec![] => "" }
case! { b_1_byte (1) vec![0x61] => "2g" }
case! { c_5_bytes (5) vec![0x51, 0x6b, 0x6f, 0xcd, 0x0f] => "ABnLTmg" }
case! { d_10_bytes (10)
    vec![0xec, 0xac, 0x89, 0xca, 0xd9, 0x39, 0x23, 0xc0, 0x23, 0x21]
        => "EJDM8drfXA6uyA"
}
case! { e_10_bytes_zero (10)
    vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        => "1111111111"
}
case! { f_10_bytes_max (10)
    vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        => "FPBt6CHo3fovdL"
}
case! { g_32_bytes (32)
    vec![
        0x18, 0xf3, 0x06, 0xdf, 0xe6, 0x99, 0xd2, 0x08, 0x5c, 0x89, 0x7b, 0x43,
        0xa4, 0xc5, 0x4f, 0xc4, 0x7d, 0x2b, 0xb7, 0x55, 0x67, 0x5b, 0xe8, 0xa7,
        0x49, 0x83, 0x68, 0x83, 0x00, 0x65, 0xd6, 0xe7
    ] => "2gPihUTjt3FJqf1VpidgrY5cZ6PuyMccGVwQHRfjMPZG"
}
case! { h_256_bytes {256}
    vec![
        0x65, 0x5f, 0x65, 0x20, 0xc4, 0xd8, 0xa5, 0x86, 0xce, 0x80, 0x1a, 0x4e,
        0x60, 0x73, 0x91, 0x40, 0x10, 0x8f, 0xd5, 0xdc, 0x5b, 0x3e, 0x8e, 0x08,
        0x47, 0x98, 0x82, 0xc6, 0x29, 0xee, 0x49, 0x8d, 0xb6, 0x41, 0xa1, 0xc6,
        0xa9, 0xd3, 0x63, 0xcb, 0xe2, 0x4e, 0x3f, 0x90, 0x78, 0x04, 0xf4, 0x49,
        0x5c, 0x4b, 0x39, 0x73, 0x9b, 0x5c, 0x4b, 0x9f, 0x23, 0xde, 0xc4, 0x8a,
        0x3d, 0xb8, 0x1a, 0x6c, 0xfd, 0x5a, 0xc1, 0xe3, 0x28, 0x9a, 0xf6, 0x72,
        0xfb, 0x2d, 0x33, 0x9d, 0xb6, 0xc4, 0x38, 0xfa, 0x8d, 0x16, 0xc9, 0x0d,
        0x00, 0xab, 0xc7, 0x9a, 0x27, 0xd2, 0x8e, 0x45, 0xdc, 0x49, 0x8d, 0xf9,
        0x80, 0x86, 0x11, 0x91, 0x86, 0x98, 0xcc, 0xc2, 0x6e, 0x85, 0xd2, 0x38,
        0xfc, 0xff, 0x66, 0xf0, 0x9d, 0x7d, 0xa5, 0x4c, 0x6f, 0x0d, 0xe5, 0xd0,
        0x60, 0x6c, 0xe7, 0x31, 0x38, 0xa0, 0x86, 0xde, 0x24, 0x28, 0x05, 0x6c,
        0x03, 0xb6, 0x21, 0xde, 0xaa, 0x8b, 0x81, 0xcc, 0xb6, 0x0e, 0x19, 0xdc,
        0xe5, 0x50, 0xb5, 0xb7, 0x6e, 0x8f, 0x22, 0xa7, 0x6f, 0x86, 0x75, 0x06,
        0xb8, 0xca, 0xa0, 0xc6, 0x29, 0x8f, 0xf6, 0xc4, 0x8b, 0x22, 0x24, 0xc0,
        0xf7, 0x09, 0x10, 0x6f, 0x10, 0x8a, 0xc2, 0x57, 0x90, 0x50, 0x62, 0x9e,
        0x95, 0x4c, 0x47, 0x79, 0xdb, 0xc9, 0x82, 0x9f, 0x45, 0xac, 0x8b, 0x31,
        0xa4, 0xfb, 0x6b, 0xdd, 0x86, 0x7f, 0x9b, 0x6f, 0x48, 0xe4, 0x34, 0x84,
        0x0c, 0x45, 0x6c, 0xfa, 0xa3, 0x14, 0x52, 0x22, 0x46, 0xf9, 0x20, 0x5f,
        0x6a, 0xb4, 0x25, 0x09, 0xb1, 0xae, 0x04, 0x3f, 0x27, 0xa0, 0xda, 0xb6,
        0x91, 0x45, 0x09, 0x37, 0xf1, 0x17, 0x2d, 0xb8, 0xa8, 0xaa, 0x5a, 0x61,
        0xf1, 0xbe, 0x08, 0x40, 0x47, 0xa8, 0x16, 0xf9, 0xb0, 0x0f, 0x6d, 0x34,
        0x62, 0x29, 0x2b, 0xb3
    ] => "\
        5gkXES6JSFLhJ3pkwQsV3MT3TBjsW5vQnAW8CwPLS1oDsJgjq8dchz994yCJHD1C16k3Pk\
        Gp8o61dMfXy1vVwXcD147ix2BXD87xcXGnzB4mxaUEvgqDonZz8xQE9XL44XvLQshJw7kp\
        54MkSPbVkxvzKdxiYHkgAjLfmx5wdyDNjPu2DUYmxRrTtjDw5QVMaqAp3fLrQ6GnXuhZmB\
        Jdj8rTprjADLM5tox6tHgyj2bm37ECxKevEapzy4nDGmZrzMubp9s58TsV1wk3LUQsRF49\
        L9NzDatxVUetHTjQennpEHEuMTU9D8GM6De44J7Sk5fnJGh614ZtmrYyFcCE3X5mdTwaxA\
    "
}
