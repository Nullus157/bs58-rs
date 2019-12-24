use anyhow::anyhow;
use std::{
    fmt,
    io::{self, Read, Write},
    str::FromStr,
};

enum Alphabet {
    Bitcoin,
    Monero,
    Ripple,
    Flickr,
    Custom([u8; 58]),
}

impl Alphabet {
    fn as_bytes(&self) -> &[u8; 58] {
        match self {
            Alphabet::Bitcoin => bs58::alphabet::BITCOIN,
            Alphabet::Monero => bs58::alphabet::MONERO,
            Alphabet::Ripple => bs58::alphabet::RIPPLE,
            Alphabet::Flickr => bs58::alphabet::FLICKR,
            Alphabet::Custom(custom) => custom,
        }
    }
}

impl FromStr for Alphabet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "bitcoin" => Alphabet::Bitcoin,
            "monero" => Alphabet::Monero,
            "ripple" => Alphabet::Ripple,
            "flickr" => Alphabet::Flickr,
            custom if custom.starts_with("custom(") && custom.ends_with(')') => {
                let alpha = custom.trim_start_matches("custom(").trim_end_matches(')');
                let bytes = alpha.as_bytes();
                if bytes.iter().any(|&c| c > 128) {
                    return Err(anyhow!("custom alphabet must be ASCII characters only"));
                }
                if bytes.len() != 58 {
                    return Err(anyhow!("custom alphabet is not 58 characters long"));
                }
                let ptr = bytes.as_ptr() as *const [u8; 58];
                Alphabet::Custom(unsafe { *ptr })
            }
            other => {
                return Err(anyhow!("'{}' is not a known alphabet", other));
            }
        })
    }
}

impl fmt::Debug for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Alphabet::Bitcoin => f.debug_tuple("Bitcoin").finish(),
            Alphabet::Monero => f.debug_tuple("Bitcoin").finish(),
            Alphabet::Ripple => f.debug_tuple("Bitcoin").finish(),
            Alphabet::Flickr => f.debug_tuple("Bitcoin").finish(),
            Alphabet::Custom(custom) => f.debug_tuple("Custom").field(&&custom[..]).finish(),
        }
    }
}

#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "bs58", setting = structopt::clap::AppSettings::ColoredHelp)]
/// A utility for encoding/decoding base58 encoded data.
struct Args {
    /// Decode input
    #[structopt(long, short = "d")]
    decode: bool,

    /// Which base58 alphabet to decode/encode with [possible values: bitcoin, monero,
    /// ripple, flickr or custom(abc...xyz)]
    #[structopt(long, short = "a", default_value = "bitcoin")]
    alphabet: Alphabet,
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let mut input = Vec::with_capacity(4096);
    io::stdin().read_to_end(&mut input)?;
    if args.decode {
        let output = bs58::decode(input)
            .with_alphabet(args.alphabet.as_bytes())
            .into_vec()?;
        io::stdout().write_all(&output)?;
    } else {
        let output = bs58::encode(input)
            .with_alphabet(args.alphabet.as_bytes())
            .into_string();
        io::stdout().write_all(output.as_bytes())?;
    }

    Ok(())
}
