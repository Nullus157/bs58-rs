use std::{
    io::{self, Read, Write},
    str::FromStr,
};

#[derive(Debug)]
enum Alphabet {
    Bitcoin,
    Monero,
    Ripple,
    Flickr,
    Custom(String),
}

impl Alphabet {
    fn as_bytes(&self) -> Result<&[u8; 58], &'static str> {
        Ok(match self {
            Alphabet::Bitcoin => bs58::alphabet::BITCOIN,
            Alphabet::Monero => bs58::alphabet::MONERO,
            Alphabet::Ripple => bs58::alphabet::RIPPLE,
            Alphabet::Flickr => bs58::alphabet::FLICKR,
            Alphabet::Custom(alphabet) => {
                let bytes = alphabet.as_bytes();
                if bytes.iter().any(|&c| c > 128) {
                    return Err("Custom alphabet must be ASCII characters only");
                }
                if bytes.len() != 58 {
                    return Err("Custom alphabet is not 58 characters long");
                }
                let ptr = bytes.as_ptr() as *const [u8; 58];
                unsafe { &*ptr }
            }
        })
    }
}

impl FromStr for Alphabet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "bitcoin" => Alphabet::Bitcoin,
            "monero" => Alphabet::Monero,
            "ripple" => Alphabet::Ripple,
            "flickr" => Alphabet::Flickr,
            custom if custom.starts_with("custom(") && custom.ends_with(')') => Alphabet::Custom(
                custom
                    .trim_start_matches("custom(")
                    .trim_end_matches(')')
                    .to_owned(),
            ),
            other => {
                return Err(format!("'{}' is not a known alphabet", other));
            }
        })
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

fn try_main(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    if args.decode {
        let mut input = String::with_capacity(4096);
        io::stdin().read_to_string(&mut input)?;
        let output = bs58::decode(input.trim())
            .with_alphabet(args.alphabet.as_bytes()?)
            .into_vec()?;
        io::stdout().write_all(&output)?;
    } else {
        let mut input = Vec::with_capacity(4096);
        io::stdin().read_to_end(&mut input)?;
        let output = bs58::encode(input)
            .with_alphabet(args.alphabet.as_bytes()?)
            .into_string();
        io::stdout().write_all(output.as_bytes())?;
    }

    Ok(())
}

#[paw::main]
fn main(args: Args) {
    pretty_env_logger::init();

    match try_main(args) {
        Ok(()) => {}
        Err(err) => log::error!("{}", err),
    }
}
