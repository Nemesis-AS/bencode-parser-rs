mod bencode;

use bencode::BEncode;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

const FILE_PATH: &str = "D:/Learn/Rust/torrent_parser/src/Hello.txt";
const OUT_FILE: &str = "D:/Learn/Rust/torrent_parser/src/out_file.txt";

fn main() {
    let res: BEncode = BEncode::parse(FILE_PATH);

    if let BEncode::Dictionary(_) = res {
        let file_path: PathBuf = PathBuf::from(OUT_FILE);
        let mut out_file =
            fs::File::create(file_path).expect("An Error Occured while creating file");
        out_file
            .write_all(format!("{:?}", res).as_bytes())
            .expect("An Error Occured while writing data to the output file!");
    }
}
