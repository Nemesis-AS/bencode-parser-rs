use bencode_parser::BEncode;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input file
    #[arg(short, long, default_value_t = String::from("./src/big-buck-bunny.torrent"))]
    input: String,
}

fn main() {
    let args: Args = Args::parse();

    let path: PathBuf = PathBuf::from(args.input);
    let bytes: Vec<u8> = fs::read(path).expect("Cannot read File!");
    let res: BEncode = BEncode::parse(bytes);

    if let BEncode::Dictionary(obj) = res {
        let info: &BEncode = obj
            .get("info")
            .expect("Cannot find Info object in the torrent");

        if let BEncode::Dictionary(info_obj) = info {
            let pieces: &BEncode = info_obj.get("pieces").expect("Cannot find Pieces!");
            if let BEncode::BinaryStr(str) = pieces {
                if str.len() % 20 != 0 {
                    panic!("Pieces Hash not valid! (The length is not a multiple of 20)");
                }
                let mut hashes: Vec<String> = Vec::new();
                let mut start_idx: usize = 0;

                while (start_idx + 20) <= str.len() {
                    let hash: String = hex::encode(&str[start_idx..start_idx + 20]);
                    hashes.push(hash);
                    start_idx += 20;
                }

                println!("{:?}", hashes);
            }
        }
    }
}
