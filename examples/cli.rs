use bencode_parser::{BEncode, Options};
use clap::Parser;
use std::fs;
use std::io::Write;
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
        println!("THe input file path does not exits!");
        return;
    }

    let path: PathBuf = PathBuf::from(&args.input);
    let bytes = fs::read(path).expect("Couldn't Read File!");

    let options: Options = Options::default();
    let res: BEncode = BEncode::parse(bytes, options);

    if let BEncode::Dictionary(_) = res {
        let file_path: PathBuf = PathBuf::from(args.output);
        let mut out_file =
            fs::File::create(file_path).expect("An Error Occured while creating file");
        out_file
            .write_all(format!("{:?}", res).as_bytes())
            .expect("An Error Occured while writing data to the output file!");
    }
}
