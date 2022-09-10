use anyhow::{anyhow, Context};
use std::{
    convert::TryInto,
    io::{self, Read, Write},
    str::FromStr,
};
use structopt::StructOpt;

#[derive(Debug)]
enum Alphabet {
    Bitcoin,
    Monero,
    Ripple,
    Flickr,
    Custom(bs58::Alphabet),
}

impl Alphabet {
    fn as_alphabet(&self) -> &bs58::Alphabet {
        match self {
            Alphabet::Bitcoin => bs58::Alphabet::BITCOIN,
            Alphabet::Monero => bs58::Alphabet::MONERO,
            Alphabet::Ripple => bs58::Alphabet::RIPPLE,
            Alphabet::Flickr => bs58::Alphabet::FLICKR,
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
                let bytes = alpha
                    .as_bytes()
                    .try_into()
                    .context("custom alphabet is not 58 characters long")?;
                Alphabet::Custom(bs58::Alphabet::new(bytes)?)
            }
            other => {
                return Err(anyhow!("'{}' is not a known alphabet", other));
            }
        })
    }
}

#[derive(Debug, StructOpt)]
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

const INITIAL_INPUT_CAPACITY: usize = 4096;

fn main() -> anyhow::Result<()> {
    let args = Args::from_iter_safe(std::env::args_os())?;

    let mut input = Vec::with_capacity(INITIAL_INPUT_CAPACITY);
    io::stdin().read_to_end(&mut input)?;

    let output: Vec<u8> = if args.decode {
        bs58::decode(String::from_utf8(input)?.trim_end())
            .with_alphabet(args.alphabet.as_alphabet())
            .try_into()?
    } else {
        bs58::encode(input)
            .with_alphabet(args.alphabet.as_alphabet())
            .into()
    };
    io::stdout().write_all(&output)?;

    Ok(())
}
