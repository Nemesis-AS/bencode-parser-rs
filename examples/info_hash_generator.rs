use bencode_parser::{BEncode, Options};
use clap::Parser;
use sha1::{Digest, Sha1};
use std::convert::TryInto;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input file
    #[arg(short, long, default_value_t = String::from("./examples/big-buck-bunny.torrent"))]
    input: String,
}

fn main() {
    let args: Args = Args::parse();

    let path: PathBuf = PathBuf::from(args.input.clone());
    if !path.exists() {
        println!("The input file path does not exits!");
        return;
    }

    let path: PathBuf = PathBuf::from(&args.input);
    let bytes = fs::read(path).expect("Couldn't Read File!");

    let options: Options = Options { parse_hex: false };
    let res: BEncode = BEncode::parse(bytes, options);

    if let BEncode::Dictionary(dict) = res {
        let info: &BEncode = dict.get("info").expect("Couldn't find Info object!");
        let encoded: String = BEncode::encode(info);

        let mut hasher = Sha1::new();
        hasher.update(encoded.as_bytes());
        let result = hasher.finalize();

        let hash_bytes: [u8; 20] = result.try_into().unwrap();
        let info_hash = hex::encode(hash_bytes);
        println!("Info Hash: {info_hash}");
    }
}
