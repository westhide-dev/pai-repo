#![feature(ascii_char_variants)]
#![feature(ascii_char)]

use std::{ascii, fs, io, path::Path};

pub struct SourceFile {
    pub bytes: Vec<u8>,
}

impl SourceFile {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let bytes = fs::read(path)?;

        // TODO: assert LF tail
        assert!(bytes.ends_with(&[ascii::Char::LineFeed.to_u8()]));

        Ok(Self { bytes })
    }

    /// # Safety
    pub fn source(&self) -> &str {
        // self.bytes must be valid UTF-8
        unsafe { std::str::from_utf8_unchecked(&self.bytes) }
    }

    pub fn content(self) -> String {
        // TODO: source file must be valid UTF-8
        unsafe { String::from_utf8_unchecked(self.bytes) }
    }
}
