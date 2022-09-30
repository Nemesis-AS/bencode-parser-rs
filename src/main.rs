use std::fs;
use std::path::PathBuf;

// const FILE_PATH: &str = "D:/Learn/Rust/torrent_parser/src/Indie.Game.The.Movie.2012.1080p.BluRay.x265-RARBG-[rarbg.to].torrent";
const FILE_PATH: &str = "D:/Learn/Rust/torrent_parser/src/Hello.txt";

fn main() {
    let path: PathBuf = PathBuf::from(FILE_PATH);
    let bytes = fs::read(path).expect("Couldn't Read File!");
    // println!("Data: {:?}", bytes);

    // let vec: Vec<u8> = vec![bytes[0]];

    // println!("{}", bytes.len());
    let mut idx: usize = 0;
    let len: usize = bytes.len();
    while idx < len - 1 {
        let ch: String = String::from_utf8(vec![bytes[idx]]).unwrap();
        idx += 1;

        match ch.as_str() {
            "i" => {
                let (new_idx, num) = parse_int(&bytes, idx - 1);
                idx = new_idx;
                println!("Parsed Integer: {}", num);
            }
            c if c.chars().next().unwrap().is_numeric() => {
                let (new_idx, out_str) = parse_str(&bytes, idx - 1);
                idx = new_idx;
                println!("Parsed String: {}", out_str);
            }
            "d" => println!("Dict"),
            "l" => println!("List"),
            "e" => continue,
            _ => (),
        }
        // println!("{}", idx);
    }
}

fn parse_int(bytes: &[u8], mut idx: usize) -> (usize, isize) {
    let mut num_str: String = String::new();

    loop {
        let ch: String = String::from_utf8(vec![bytes[idx]]).unwrap();
        idx += 1;
        match ch.as_str() {
            c if c.chars().next().unwrap().is_numeric() => {
                num_str.push_str(c);
            }
            "-" => num_str.push_str(&ch.clone()),
            "e" => break,
            "i" => continue,
            _ => break,
        }
    }

    let s = num_str.parse::<isize>();
    match s {
        Ok(num) => (idx, num),
        Err(_err) => {
            println!("An Error Occured while parsing Int");
            (idx, -1)
        }
    }
}

fn parse_str(bytes: &[u8], mut idx: usize) -> (usize, String) {
    let mut len_str: String = String::new();

    // This loop determines the length of the string
    loop {
        let ch: String = String::from_utf8(vec![bytes[idx]]).unwrap();
        idx += 1;
        match ch.as_str() {
            c if c.chars().next().unwrap().is_numeric() => {
                len_str.push_str(c);
            }
            ":" => break,
            _ => break,
        }
    }

    let len: usize = len_str
        .parse::<usize>()
        .unwrap_or_else(|_err| panic!("Invalid String Length found at column {}", idx));

    let out_str = String::from_utf8(bytes[idx..idx + len].to_vec()).unwrap();

    (idx + len, out_str)
}

// fn parse_dictionary() {}

// fn parse_list() {}
