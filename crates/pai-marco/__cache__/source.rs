#![feature(prelude_import)]
//! Inspire by
//! - [cargo-expand](https://github.com/dtolnay/cargo-expand)
//! - [trybuild](https://github.com/dtolnay/trybuild)
//! - [macrotest](https://github.com/eupn/macrotest)
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;

use insta::assert_snapshot;
use pai_error::PResult;
use pai_file::SourceFile;
use quote::ToTokens;
use syn;
use syn_select;

mod source {





    pub mod unit {
        use pai_marco::ts_attribute;
        pub fn foo() {}
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "main"]
pub const main: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("main"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "crates\\pai-marco\\tests\\run.rs",
            start_line: 16usize,
            start_col: 4usize,
            end_line: 16usize,
            end_col: 8usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(main())),
    };
#[allow(dead_code)]
fn main() -> PResult<()> {
    let source_file = SourceFile::read("__cache__/source.rs")?;
    let syn_file = syn::parse_str(source_file.source())?;
    let code =
        syn::File {
                    shebang: None,
                    attrs: Vec::new(),
                    items: syn_select::select("source::unit", &syn_file)?,
                }.into_token_stream().to_string();
    {
        ::insta::_macro_support::assert_snapshot(::insta::_macro_support::AutoName.into(),
                &code, "D:\\Code\\pai-repo\\crates\\pai-marco",
                {
                    fn f() {}
                    fn type_name_of_val<T>(_: T) -> &'static str {
                        std::any::type_name::<T>()
                    }
                    let mut name =
                        type_name_of_val(f).strip_suffix("::f").unwrap_or("");
                    while let Some(rest) = name.strip_suffix("::{{closure}}") {
                        name = rest;
                    }
                    name
                }, "run", "crates\\pai-marco\\tests\\run.rs", 29u32,
                "code").unwrap()
    };
    Ok(())
}
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&main])
}
