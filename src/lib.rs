//! > **A Bencode Parser written in Rust**
//!
//! ## Example
//! ```rust
//! use bencode::{BEncode, Options};
//!
//! let path: PathBuf = PathBuf::from("./examples/big-buck-bunny.torrent");
//! let bytes = fs::read(path).expect("Couldn't Read File!");
//! let options: Options = Options::default();
//! let res: BEncode = BEncode::parse(bytes, options);
//! println!("Decoded Object: {:?}", res);
//! ```

mod options;

pub use options::Options;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;

/// The BEncode Object.
/// This enum wraps the data types supported by bencode objects, with an addition of `String`.
/// The `String` variant holds the `BinaryStr` which are valid UTF-8 strings.
pub enum BEncode {
    /// The `Int` variant holds the integers parsed from bencode
    Int(isize),
    /// The `String` variant holds parsed bencode ByteStrings that have valid UTF-8 characters
    String(String),
    /// The `List` variant holds parsed bencode Lists. They can hold any of the bencode types as children
    List(Vec<BEncode>),
    /// The `Dictionary` variant holds parsed bencode Dictionaries. They are similar to `BTreeMap`, but the keys can only be [`BEncode::String`]. Though the value can be of any of the bencode types
    Dictionary(BTreeMap<String, BEncode>),
    /// The `BinaryStr` variant holds parsed bencode ByteStrings that do not have valid UTF-8 characters. They are useful for dealing with the `pieces` property of a torrent file as they contain binary strings.
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
        };

        write!(f, "{}", output)
    }
}

impl BEncode {
    /// This function returns the parsed [`BEncode`] object, given a valid path to a file containing bencode.
    /// returns a `Bencode::Int(-1)` if the bencode cannot be parsed
    pub fn parse(bytes: Vec<u8>, options: Options) -> Self {
        // =====================STATE VARIABLES==========================
        let mut parents: Vec<BEncode> = Vec::new();
        let mut dict_keys: Vec<String> = Vec::new();
        // ==============================================================

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
                                }
                                parents.push(parent);
                            }
                            _ => (),
                        }
                    }
                }
                // String
                c if c.chars().next().unwrap().is_numeric() => {
                    let (new_idx, out_str) = Self::parse_str(&bytes, idx - 1, options.parse_hex);
                    idx = new_idx;
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
                    parents.push(BEncode::Dictionary(BTreeMap::new()));
                }
                "e" => match parents.len().cmp(&1) {
                    Ordering::Greater => {
                        let parent: BEncode = parents.pop().unwrap();

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
                                }
                                parents.push(root);
                            }
                            _ => (),
                        }
                    }
                    Ordering::Equal => {
                        let root: BEncode = parents.pop().unwrap();
                        return root;
                    }
                    _ => (),
                },
                _ => println!("Nothing"),
            }
        }

        BEncode::Int(-1)
    }

    /// Encodes the given [`BEncode`] object recursively to bencode and returns the encoded [`String`]
    pub fn encode(object: &Self) -> String {
        let mut output: String = String::new();

        match object {
            Self::Int(_) => {
                output.push_str(&Self::encode_shallow_data(object));
            }
            Self::String(_) => {
                output.push_str(&Self::encode_shallow_data(object));
            }
            Self::BinaryStr(_) => {
                output.push_str(&Self::encode_shallow_data(object));
            }
            Self::List(_) => {
                output.push_str(&Self::encode_list(object));
            }
            Self::Dictionary(_) => {
                output.push_str(&Self::encode_dict(object));
            }
        }

        output
    }

    /// Internal function to encode non-collection data types - [`BEncode::Int`], [`BEncode::String`] and [`BEncode::BinaryStr`]
    fn encode_shallow_data(data: &Self) -> String {
        match data {
            Self::Int(num) => format!("i{}e", num),
            Self::String(string) => format!("{}:{}", string.len(), string),
            Self::BinaryStr(bin) => unsafe {
                let bytes = bin.clone();
                let out: String = String::from_utf8_unchecked(bytes);
                format!("{}:{}", out.len(), out)
            },
            _ => String::from(""),
        }
    }

    /// Internal function to encode [`BEncode::List`] objects
    fn encode_list(object: &Self) -> String {
        let mut output: String = String::new();

        if let Self::List(list) = object {
            output.push('l');
            for item in list {
                output.push_str(Self::encode(item).as_str());
            }
            output.push('e');
        }

        output
    }

    /// Internal function to encode [`BEncode::Dictionary`] objects
    fn encode_dict(object: &Self) -> String {
        let mut output: String = String::new();

        if let Self::Dictionary(dict) = object {
            output.push('d');
            for (key, item) in dict {
                output.push_str(format!("{}:{}", key.len(), key).as_str());
                output.push_str(Self::encode(item).as_str());
            }
            output.push('e');
        }

        output
    }

    /// This function is used to push items inside bencode Lists[`BEncode::List`] and Dictionaries[`BEncode::Dictionary`]
    /// The addition happens in place so it does not return anything
    /// This will not work for [`BEncode::Int`], [`BEncode::String`] or [`BEncode::BinaryStr`]
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

    /// Internal function to parse a bencode Integer
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

    /// Internal function to parse a bencode ByteString
    fn parse_str(bytes: &[u8], mut idx: usize, parse_hex: bool) -> (usize, BEncode) {
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
            if parse_hex {
                return (idx + len, BEncode::String(hex::encode(byte_slice)));
            }
            return (idx + len, BEncode::BinaryStr(byte_slice.to_vec()));
        }

        (idx + len, BEncode::String(out_str))
    }
}
