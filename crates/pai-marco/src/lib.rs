use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn ts_attribute(_input: TokenStream, _annotated_item: TokenStream) -> TokenStream {
    _annotated_item
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(true);
    }
}
