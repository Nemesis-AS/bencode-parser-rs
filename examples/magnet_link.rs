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

    /// Path to the output file
    #[arg(short, long, default_value_t = String::from("./target/bencode_out_file.txt"))]
    output: String,
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
        let mut magnet: String = String::from("magnet:?xt=urn:btih:");

        let info: &BEncode = dict.get("info").expect("Couldn't find Info object!");
        let info_hash: String = get_info_hash(info);
        magnet.push_str(&info_hash);

        // ========================== GET DISPLAY NAME ======================================
        if let BEncode::Dictionary(info_dict) = info {
            let name_benc: BEncode = BEncode::String(String::new());
            let name_obj: &BEncode = info_dict.get("name").unwrap_or(&name_benc);

            if let BEncode::String(name) = name_obj {
                let display_name: String = format!("&dn={}", name.clone().replace(' ', "+"));
                magnet.push_str(&display_name);
                // println!("{magnet}");
            }
        }
        // ==================================================================================

        // ========================== GET TRACKERS LIST =====================================
        let announce_list_benc: &BEncode = dict.get("announce-list").expect("No Trackers Found!");
        if let BEncode::List(announce_list) = announce_list_benc {
            for list in announce_list {
                if let BEncode::List(l) = list {
                    if let BEncode::String(tracker) = &l[0] {
                        let tracker_string: String = format!(
                            "&tr={}",
                            tracker.clone().replace(':', "%3A").replace('/', "%2F")
                        );
                        magnet.push_str(&tracker_string);
                    }
                }
            }
        }
        // ==================================================================================

        // ========================== GET WEB SEEDS =========================================
        let dummy_list: BEncode = BEncode::List(vec![]);
        let urls_benc: &BEncode = dict.get("url-list").unwrap_or(&dummy_list);

        if let BEncode::List(urls) = urls_benc {
            for link_benc in urls {
                if let BEncode::String(url) = link_benc {
                    let web_string: String = format!(
                        "&ws={}",
                        url.clone().replace(':', "%3A").replace('/', "%2F")
                    );
                    magnet.push_str(&web_string);
                }
            }
        }
        // ==================================================================================

        println!("{magnet}");
    }
}

fn get_info_hash(info: &BEncode) -> String {
    let encoded: String = BEncode::encode(info);

    let mut hasher = Sha1::new();
    hasher.update(encoded.as_bytes());
    let result = hasher.finalize();

    let hash_bytes: [u8; 20] = result.try_into().unwrap();
    hex::encode(hash_bytes)
}
