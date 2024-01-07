use std::{fs, io};

pub struct SourceFile<'s> {
    pub path: &'s str,
    pub bytes: Vec<u8>,
}

impl<'s> SourceFile<'s> {
    pub fn read(path: &'s str) -> Result<Self, io::Error> {
        let bytes = fs::read(path)?;

        // TODO: assert LF tail
        assert!(bytes.ends_with(&[10]));

        Ok(Self { path, bytes })
    }

    /// # Safety
    pub fn source(&self) -> &str {
        // TODO: source file must be valid UTF-8
        unsafe { std::str::from_utf8_unchecked(&self.bytes) }
    }
}
