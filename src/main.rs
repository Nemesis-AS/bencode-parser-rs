use std::cmp::Ordering;
use std::path::PathBuf;
use std::{fmt, fs};

// const FILE_PATH: &str = "D:/Learn/Rust/torrent_parser/src/Indie.Game.The.Movie.2012.1080p.BluRay.x265-RARBG-[rarbg.to].torrent";
const FILE_PATH: &str = "D:/Learn/Rust/torrent_parser/src/Hello.txt";

#[derive(Clone)]
enum BEncode {
    Int(isize),
    String(String),
    List(Vec<BEncode>),
}

impl fmt::Debug for BEncode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Self::Int(value) => value.to_string(),
            Self::String(value) => value.clone(),
            Self::List(value) => format!("{:?}", value),
            // _ => String::from(""),
        };

        write!(f, "{}", output)
    }
}

impl BEncode {
    fn push(&mut self, item: BEncode) {
        match self {
            Self::Int(_) => {
                println!("Cannot insert BEncode Object inside Integer!")
            }
            Self::String(_) => {
                println!("Cannot insert BEncode Object inside String!")
            }
            Self::List(value) => {
                value.push(item);
            }
        }
    }
}

fn main() {
    let path: PathBuf = PathBuf::from(FILE_PATH);
    let bytes = fs::read(path).expect("Couldn't Read File!");

    // Parents
    let mut parents: Vec<BEncode> = Vec::new();

    let mut idx: usize = 0;
    let len: usize = bytes.len();
    while idx < len {
        let ch: String = String::from_utf8(vec![bytes[idx]]).unwrap();
        idx += 1;

        match ch.as_str() {
            // Integer
            "i" => {
                let (new_idx, num) = parse_int(&bytes, idx - 1);
                idx = new_idx;
                println!("Parsed Integer: {:?}", num);
                if !parents.is_empty() {
                    let mut parent: BEncode = parents.pop().unwrap();
                    parent.push(num);
                    parents.push(parent);
                }
            }
            // String
            c if c.chars().next().unwrap().is_numeric() => {
                let (new_idx, out_str) = parse_str(&bytes, idx - 1);
                idx = new_idx;
                println!("Parsed String: {:?}", out_str);
                if !parents.is_empty() {
                    let mut parent: BEncode = parents.pop().unwrap();
                    parent.push(out_str);
                    parents.push(parent);
                }
            }
            // List
            "l" => {
                parents.push(BEncode::List(Vec::new()));
            }
            // Dictionary
            "d" => println!("Dict"),
            "e" => {
                match parents.len().cmp(&1) {
                    Ordering::Greater => {
                        let parent: BEncode = parents.pop().unwrap();
                        println!("Parsed BEncode Object: {:?}", parent);

                        let mut root: BEncode = parents.pop().unwrap();
                        root.push(parent);
                        parents.push(root);
                    }
                    Ordering::Equal => {
                        let root: BEncode = parents.pop().unwrap();
                        // return root;
                        println!("Root Object: {:?}", root);
                    }
                    _ => (),
                }
            }
            _ => println!("Nothing"),
        }
    }
}

fn parse_int(bytes: &[u8], mut idx: usize) -> (usize, BEncode) {
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
        Ok(num) => (idx, BEncode::Int(num)),
        Err(_err) => {
            println!("An Error Occured while parsing Int");
            (idx, BEncode::Int(-1))
        }
    }
}

fn parse_str(bytes: &[u8], mut idx: usize) -> (usize, BEncode) {
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

    (idx + len, BEncode::String(out_str))
}
