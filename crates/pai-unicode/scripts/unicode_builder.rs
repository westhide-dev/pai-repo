use std::{env, fmt, fs, iter::FromIterator, num::ParseIntError, str::FromStr};

use pai_error::PResult;
use pai_file::SourceFile;

const ATUOGEN_DIR: &str = env!("ENV_PAI_UNICODE_AUTOGEN_DIR");
const DOWNLOAD_DIR: &str = env!("ENV_PAI_UNICODE_DOWNLOAD_DIR");

const PROPERTIES_FILE: &str = "DerivedCoreProperties.txt";

pub struct UnicodeBuilder {
    content: String,
}

impl UnicodeBuilder {
    pub fn try_new() -> PResult<Self> {
        let properties_file = format!("{DOWNLOAD_DIR}/{PROPERTIES_FILE}");

        Ok(Self {
            content: SourceFile::read(properties_file)?.content(),
        })
    }

    pub fn build(self, properties: &[&str]) -> PResult<()> {
        properties.iter().try_for_each(|property| {
            let table_file = format!("{ATUOGEN_DIR}/{property}.table");
            CodePointTable::new(property, &self.content)?.sink(&table_file)
        })
    }
}

/// [CodePoint](https://www.unicode.org/glossary/#code_point)
struct CodePointRange {
    lo: u32,
    hi: u32,
}

impl CodePointRange {
    fn new(lo: &str, hi: &str) -> Result<Self, ParseIntError> {
        Ok(Self {
            lo: u32::from_str_radix(lo, 16)?,
            hi: u32::from_str_radix(hi, 16)?,
        })
    }
}

impl FromStr for CodePointRange {
    type Err = ParseIntError;

    fn from_str(range: &str) -> Result<Self, Self::Err> {
        match range.split_once("..") {
            Some((lo, hi)) => Self::new(lo, hi),
            None => Self::new(range, range),
        }
    }
}

impl From<CodePointRange> for String {
    fn from(CodePointRange { lo, hi }: CodePointRange) -> Self {
        fmt::format(format_args_nl!("(0x{lo:05X},0x{hi:05X})"))
    }
}

struct CodePointTable {
    items: Vec<CodePointRange>,
}

impl FromIterator<CodePointRange> for CodePointTable {
    fn from_iter<T: IntoIterator<Item = CodePointRange>>(iter: T) -> Self {
        let mut items = Vec::<CodePointRange>::new();

        for range @ CodePointRange { lo, hi } in iter {
            match items.last_mut() {
                Some(prev) => {
                    assert!(prev.hi < lo && lo <= hi);

                    if lo - prev.hi == 1 {
                        prev.hi = hi
                    } else {
                        items.push(range)
                    }
                },
                None => items.push(range),
            }
        }

        Self { items }
    }
}

impl CodePointTable {
    #[rustfmt::skip]
    fn new(property: &str, content: &str) -> Result<CodePointTable, ParseIntError> {
        content
        .lines()
        .filter_map(|line| {
            if line.chars().next()? == '#' {
                return None
            }

            let (range, name) = line.split('#').next()?.split_once(';')?;

            if name.trim() == property {
                Some(range.trim())
            } else {
                None
            }
        })
        .map(CodePointRange::from_str)
        .collect()
    }

    fn sink(self, path: &str) -> PResult<()> {
        let contents: String = self.items.into_iter().map(String::from).collect();
        Ok(fs::write(path, contents)?)
    }
}
