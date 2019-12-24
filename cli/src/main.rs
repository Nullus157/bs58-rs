use std::io::{self, Read, Write};

#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "bs58", setting = structopt::clap::AppSettings::ColoredHelp)]
/// A utility for encoding/decoding base58 encoded data.
struct Args {
    /// Decode input
    #[structopt(long, short = "d")]
    decode: bool,
}

fn try_main(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    if args.decode {
        let mut input = String::with_capacity(4096);
        io::stdin().read_to_string(&mut input)?;
        let output = bs58::decode(input.trim()).into_vec()?;
        io::stdout().write_all(&output)?;
    } else {
        let mut input = Vec::with_capacity(4096);
        io::stdin().read_to_end(&mut input)?;
        let output = bs58::encode(input).into_string();
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
