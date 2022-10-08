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

    /// Length of each hash. Default is 40 instead of 20 since coverting binary to hex returns twice as many characters
    #[arg(short, long, default_value_t = 40)]
    length: usize,
}

fn main() {
    let args: Args = Args::parse();

    let path: PathBuf = PathBuf::from(args.input);
    let bytes: Vec<u8> = fs::read(path).expect("Cannot read File!");

    let options: Options = Options::default();
    let res: BEncode = BEncode::parse(bytes, options);

    if let BEncode::Dictionary(obj) = res {
        let info: &BEncode = obj
            .get("info")
            .expect("Cannot find Info object in the torrent");

        if let BEncode::Dictionary(info_obj) = info {
            let pieces: &BEncode = info_obj.get("pieces").expect("Cannot find Pieces!");
            if let BEncode::String(str) = pieces {
                if str.len() % args.length != 0 {
                    panic!(
                        "Pieces Hash not valid! (The length is not a multiple of {})",
                        args.length
                    );
                }
                let mut hashes: Vec<String> = Vec::new();
                let mut start_idx: usize = 0;

                while (start_idx + args.length) <= str.len() {
                    let hash: String = str[start_idx..start_idx + args.length].to_string();
                    hashes.push(hash);
                    start_idx += args.length;
                }

                println!("{:?}", hashes);
            }
        }
    }
}
