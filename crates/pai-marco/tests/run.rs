//! Inspire by
//! - [cargo-expand](https://github.com/dtolnay/cargo-expand)
//! - [trybuild](https://github.com/dtolnay/trybuild)
//! - [macrotest](https://github.com/eupn/macrotest)

use insta::assert_snapshot;
use pai_error::PResult;
use pai_file::SourceFile;
use quote::ToTokens;
use syn;
use syn_select;

mod source;

#[test]
fn main() -> PResult<()> {
    let syn_file = syn::parse_str(SourceFile::read("__cache__/code.rs")?.source())?;

    let code = syn::File {
        shebang: None,
        attrs: Vec::new(),
        items: syn_select::select("source", &syn_file)?,
    }
    .into_token_stream()
    .to_string();

    Ok(assert_snapshot!(code))
}
