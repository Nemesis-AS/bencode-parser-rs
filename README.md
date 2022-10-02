> :warning:This project is still a WIP :warning:

# Bencode Parser

A bencode parser written in Rust

## Getting Started

To get started, clone the repository to your machine. Open the terminal in the root directory of the project and run
```shell
cargo b
.\\target\\debug\\torrent_parser.exe -i <PATH-TO-INPUT-FILE> -o <PATH-TO-OUTPUT-FILE>
```

## What is `[Binary String]`?

The torrent files have a property called `pieces` where the `SHA-1` hashes of all the pieces of the torrent are stored, which is in the form of a binary string and not UTF-8. Parsing it as a String would make the program unsafe as the String will not be checked before parsing. To prevent this, all the binary strings are used in the binary(`Vec<u8>`) for itself.