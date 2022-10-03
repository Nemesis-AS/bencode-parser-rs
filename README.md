> :warning:This project is still a WIP :warning:

# Bencode Parser

A bencode parser written in Rust

## Getting Started

```rust
let path: PathBuf = PathBuf::from("./src/Hello.txt");
let bytes = fs::read(path).expect("Couldn't Read File!");
let res: BEncode = BEncode::parse(bytes);
println!("Decoded Object: {:?}", res);
```

## What is `[Binary String]`?

The torrent files have a property called `pieces` where the `SHA-1` hashes of all the pieces of the torrent are stored, which is in the form of a binary string and not UTF-8. Parsing it as a String would make the program unsafe as the String will not be checked before parsing. To prevent this, all the binary strings are used in the binary(`Vec<u8>`) for itself.