use crate::common::bsearch_range_table;

// const ATUOGEN_DIR: &str = env!("ENV_PAI_UNICODE_AUTOGEN_DIR");
// include!(concat!(env!("ATUOGEN_DIR"), "/ID_Start.rs"));

// TODO:
const ID_START: &[(char, char)] = &[];

pub fn is_ident_start(ch: char) -> bool {
    bsearch_range_table(ch, ID_START)
}

// TODO:
const ID_CONTINUE: &[(char, char)] = &[];

pub fn is_ident_continue(ch: char) -> bool {
    bsearch_range_table(ch, ID_CONTINUE)
}
