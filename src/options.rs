//! The `Options` struct contains the configurable options that the parser can take
pub struct Options {
    /// `parse_hex` tells the parser if the hex data from pieces should be parsed. Default value: `true`
    pub parse_hex: bool,
}

impl Options {
    pub fn default() -> Self {
        Self { parse_hex: true }
    }
}
