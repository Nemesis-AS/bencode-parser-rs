use bencode_parser::{BEncode, Options};
use clap::Parser;
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

    let path: PathBuf = PathBuf::from(args.input);
    let bytes: Vec<u8> = fs::read(path).expect("Cannot read File!");

    let options: Options = Options::default();
    let res: BEncode = BEncode::parse(bytes, options);

    // The length is 20 in torrent files, but parsing 1 byte to hex returns 2 characters, so the length also has to be doubled
    let hash_length: usize = 40;

    if let BEncode::Dictionary(obj) = res {
        let info: &BEncode = obj
            .get("info")
            .expect("Cannot find Info object in the torrent");

        if let BEncode::Dictionary(info_obj) = info {
            let pieces: &BEncode = info_obj.get("pieces").expect("Cannot find Pieces!");
            if let BEncode::String(str) = pieces {
                if str.len() % hash_length != 0 {
                    panic!(
                        "Pieces Hash not valid! (The length is not a multiple of {})",
                        hash_length
                    );
                }
                let mut hashes: Vec<String> = Vec::new();
                let mut start_idx: usize = 0;

                while (start_idx + hash_length) <= str.len() {
                    let hash: String = str[start_idx..start_idx + hash_length].to_string();
                    hashes.push(hash);
                    start_idx += hash_length;
                }

                println!("{:?}", hashes);
            }
        }
    }
}
