use leb128::{ReadLeb128, WriteLeb128};
use main_error::MainError;
use std::io::{self, BufRead};
use structopt::StructOpt;

/// Utility to encode/decode integers to/from LEB128.
#[derive(StructOpt, Debug)]
#[structopt(
    name = "leb128",
    raw(setting = "structopt::clap::AppSettings::AllowLeadingHyphen")
)]
struct Options {
    /// Decode LEB128 to integer (default: encode integer to LEB128)
    #[structopt(short = "d", long = "decode", display_order = 1)]
    decode: bool,

    /// Encode/decode signed LEB128 (default: unsigned LEB128)
    #[structopt(short = "s", long = "signed", display_order = 2)]
    signed: bool,

    /// Integer (decimal) to encode or LEB128 (hex bytes) to decode.
    /// {n}Read from standard input if not given.
    /// {n}Hex bytes must have two digits each (one per nibble).
    /// {n}Hex bytes may be prefixed with '0x' and may contain spaces.
    #[structopt(name = "INPUT")]
    input: Vec<String>,
}

fn main() -> Result<(), MainError> {
    let opt = Options::from_args();

    let input = if opt.input.len() > 0 {
        // concatenate a split-up number
        opt.input.concat()
    } else {
        // read input from stdin if not given
        io::stdin().lock().lines().next().unwrap()?
    };

    if opt.decode {
        let input = input.replace("0x", "").replace(" ", "");
        let buf = hex::decode(input)?;
        let input_num_bytes = buf.len();
        let mut cursor = io::Cursor::new(buf);
        let result = if opt.signed {
            let int: i64 = cursor.read_leb128()?;
            int.to_string()
        } else {
            let int: u64 = cursor.read_leb128()?;
            int.to_string()
        };
        if cursor.position() < input_num_bytes as u64 {
            Err("provided LEB128 has too many bytes")?;
        }
        println!("{}", result)
    } else {
        let mut buf = Vec::new();
        if opt.signed {
            let input: i64 = input.parse()?;
            buf.write_leb128(input)?;
        } else {
            let input: u64 = input.parse()?;
            buf.write_leb128(input)?;
        }
        println!("0x{}", hex::encode(buf))
    }

    Ok(())
}
