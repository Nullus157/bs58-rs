use alphabet;

/// A builder for setting up the alphabet and output of a base58 encode.
#[allow(missing_debug_implementations)]
pub struct EncodeBuilder<'a, I: AsRef<[u8]>> {
    input: I,
    alpha: &'a [u8; 58],
}

impl<'a, I: AsRef<[u8]>> EncodeBuilder<'a, I> {
    /// Change the alphabet that will be used for encoding.
    pub fn with_alphabet(self, alpha: &[u8; 58]) -> EncodeBuilder<I> {
        EncodeBuilder { input: self.input, alpha: alpha }
    }

    /// Encode into a new owned string.
    pub fn into_string(self) -> String {
        let input = self.input.as_ref();
        let mut output = String::with_capacity((input.len() / 5 + 1) * 8);
        encode_into(input, &mut output, self.alpha);
        output
    }

    /// Encode into the given string, any existing data will be cleared.
    pub fn into(self, output: &mut String) {
        encode_into(self.input.as_ref(), output, self.alpha);
    }
}

/// Setup encoder for the given bytes using the [default alphabet][].
/// [default alphabet]: alphabet/constant.DEFAULT.html
pub fn encode<I: AsRef<[u8]>>(input: I) -> EncodeBuilder<'static, I> {
    EncodeBuilder { input: input, alpha: alphabet::DEFAULT }
}

/// Encode given bytes into given string using the given alphabet, any existing
/// data will be cleared.
pub fn encode_into(input: &[u8], output: &mut String, alpha: &[u8; 58]) {
    output.clear();
    let mut output = unsafe { output.as_mut_vec() };

    for &val in input.iter() {
        let mut carry = val as usize;
        for byte in &mut output[..] {
            carry += (*byte as usize) << 8;
            *byte = (carry % 58) as u8;
            carry /= 58;
        }
        while carry > 0 {
            output.push((carry % 58) as u8);
            carry /= 58;
        }
    }

    for &val in input.iter() {
        if val == 0 {
            output.push(0);
        } else {
            break;
        }
    }

    for val in &mut output[..] {
        *val = alpha[*val as usize];
    }

    output.reverse();
}

// Subset of test cases from https://github.com/cryptocoinjs/base-x/blob/master/test/fixtures.json
#[cfg(test)]
mod tests {
    use encode;

    #[test]
    fn tests() {
        for &(val, s) in super::super::TEST_CASES.iter() {
            assert_eq!(s, encode(val).into_string())
        }
    }
}
