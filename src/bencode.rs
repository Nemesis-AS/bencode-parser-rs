use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fmt, fs};

pub enum BEncode {
    Int(isize),
    String(String),
    List(Vec<BEncode>),
    Dictionary(HashMap<String, BEncode>),
    BinaryStr(Vec<u8>),
}

impl fmt::Debug for BEncode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Self::Int(value) => value.to_string(),
            Self::String(value) => value.clone(),
            Self::List(value) => format!("{:?}", value),
            Self::Dictionary(value) => format!("{:?}", value),
            Self::BinaryStr(_) => "[Binary String]".to_string(),
            // _ => String::from(""),
        };

        write!(f, "{}", output)
    }
}

impl BEncode {
    pub fn parse(file_path: &str) -> Self {
        let path: PathBuf = PathBuf::from(file_path);
        let bytes = fs::read(path).expect("Couldn't Read File!");

        // STATE VARIABLES

        // Parents
        let mut parents: Vec<BEncode> = Vec::new();
        let mut dict_keys: Vec<String> = Vec::new();

        let mut idx: usize = 0;
        let len: usize = bytes.len();
        while idx < len {
            let ch: String = String::from_utf8(vec![bytes[idx]]).unwrap();
            idx += 1;

            match ch.as_str() {
                // Integer
                "i" => {
                    let (new_idx, num) = Self::parse_int(&bytes, idx - 1);
                    idx = new_idx;
                    // println!("Parsed Integer: {:?}", num);
                    if !parents.is_empty() {
                        let mut parent: BEncode = parents.pop().unwrap();

                        match parent {
                            BEncode::List(_) => {
                                parent.push(num, None);
                                parents.push(parent);
                            }
                            BEncode::Dictionary(_) => {
                                if dict_keys.is_empty() {
                                    println!(
                                        "[BEncode Error] Cannot use Int as key for Dictionary!"
                                    );
                                } else {
                                    parent.push(num, Some(dict_keys.pop().unwrap()));
                                    // dict_key = String::new();
                                }
                                parents.push(parent);
                            }
                            _ => (),
                        }
                    }
                }
                // String
                c if c.chars().next().unwrap().is_numeric() => {
                    let (new_idx, out_str) = Self::parse_str(&bytes, idx - 1);
                    idx = new_idx;
                    // println!("Parsed String: {:?}", out_str);
                    if !parents.is_empty() {
                        let mut parent: BEncode = parents.pop().unwrap();

                        match parent {
                            BEncode::List(_) => {
                                parent.push(out_str, None);
                                parents.push(parent);
                            }
                            BEncode::Dictionary(_) => {
                                if dict_keys.len() <= parents.len() {
                                    dict_keys.push(format!("{:?}", out_str));
                                } else {
                                    parent.push(out_str, Some(dict_keys.pop().unwrap()));
                                    // dict_key = String::new();
                                }
                                parents.push(parent);
                            }
                            _ => (),
                        }
                    }
                }
                // List
                "l" => {
                    parents.push(BEncode::List(Vec::new()));
                }
                // Dictionary
                "d" => {
                    parents.push(BEncode::Dictionary(HashMap::new()));
                }
                "e" => {
                    match parents.len().cmp(&1) {
                        Ordering::Greater => {
                            let parent: BEncode = parents.pop().unwrap();
                            // println!("Parsed BEncode Object: {:?}", parent);

                            let mut root: BEncode = parents.pop().unwrap();

                            match root {
                                BEncode::List(_) => {
                                    root.push(parent, None);
                                    parents.push(root);
                                }
                                BEncode::Dictionary(_) => {
                                    if dict_keys.is_empty() {
                                        println!("[BEncode Error] Cannot use Non-String BEncode Object as Dictionary Key!");
                                    } else {
                                        root.push(parent, Some(dict_keys.pop().unwrap()));
                                        // dict_key = String::new();
                                    }
                                    parents.push(root);
                                }
                                _ => (),
                            }
                        }
                        Ordering::Equal => {
                            let root: BEncode = parents.pop().unwrap();
                            return root;
                            // println!("Root Object: {:?}", root);
                        }
                        _ => (),
                    }
                }
                _ => println!("Nothing"),
            }
        }

        BEncode::Int(-1)
    }

    fn push(&mut self, item: BEncode, key: Option<String>) {
        match self {
            Self::Int(_) => {
                println!("Cannot insert BEncode Object inside Integer!")
            }
            Self::String(_) => {
                println!("Cannot insert BEncode Object inside String!")
            }
            Self::BinaryStr(_) => {
                println!("Cannot insert BEncode Object inside String (Yet)!")
            }
            Self::List(value) => {
                value.push(item);
            }
            Self::Dictionary(value) => {
                if key.is_none() {
                    println!("No key provided!");
                    return;
                }
                value.insert(key.unwrap(), item);
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

        let byte_slice: Vec<u8> = bytes[idx..idx + len].to_vec();
        let out_str = String::from_utf8(byte_slice.clone()).unwrap_or_else(|_e| "".to_string());

        if out_str.is_empty() {
            // unsafe {
            //     // let bin = String::from_utf8_unchecked(byte_slice);
            //     return (idx + len, BEncode::BinaryStr(byte_slice.to_vec()));
            // }
            return (idx + len, BEncode::BinaryStr(byte_slice.to_vec()));
        }

        (idx + len, BEncode::String(out_str))
    }
}
