use std::cmp::Ordering;

pub fn bsearch_range_table(ch: char, r: &[(char, char)]) -> bool {
    r.binary_search_by(|&(lo, hi)| {
        if lo > ch {
            Ordering::Greater
        } else if hi < ch {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    })
    .is_ok()
}

pub const fn char_width(byte: u8) -> usize {
    core::str::utf8_char_width(byte)
}
