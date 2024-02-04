#![feature(proc_macro_quote)]

//! [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
//! [Macros by Example](https://doc.rust-lang.org/reference/macros.html)

use proc_macro::{quote, TokenStream};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn lookup_table(attr: TokenStream, item: TokenStream) -> TokenStream {
    dbg!(&item);
    dbg!(&attr);
    // parse_macro_input!(item as ItemFn);

    item
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert!(true);
    }
}
