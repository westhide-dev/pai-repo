//! TODO: Port [unicode-ident](https://github.com/dtolnay/unicode-ident)
//! [Glossary of Unicode Terms](https://www.unicode.org/glossary/)
//! [Unicode CodePoints](https://codepoints.net/)

#![allow(internal_features)]
#![feature(str_internals)]

#[macro_use]
pub mod macros;

pub mod common;
pub mod decode;
pub mod unicode;
