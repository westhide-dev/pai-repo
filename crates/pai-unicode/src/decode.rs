pub trait UTF8Decode<T> {
    /// | Char. number range      | UTF-8 octet sequence                |
    /// |-------------------------|-------------------------------------|
    /// | Hex Unicode             | Binary UTF8 byte                    |
    /// | U+0000...U+007F         | 0xxxxxxx                            |
    /// | U+0080...U+07FF         | 110xxxxx 10yyyyyy                   |
    /// | U+0800...U+FFFF         | 1110xxxx 10yyyyyy 10zzzzzz          |
    /// | U+10000...U+10FFFF      | 11110xxx 10yyyyyy 10zzzzzz 10wwwwww |
    fn decode(self) -> T;
}

const BIT_MASKS: [u8; 8] = [
    0b0000_0000,
    0b0000_0001,
    0b0000_0011,
    0b0000_0111,
    0b0000_1111,
    0b0001_1111,
    0b0011_1111,
    0b0111_1111,
];

macro_rules! char {
    ($code_point:expr) => {
        unsafe { char::from_u32_unchecked($code_point as u32) }
    };
}

impl UTF8Decode<char> for *const u8 {
    /// [UTF8](https://tools.ietf.org/html/rfc3629)
    ///
    /// Inspired by
    /// - [core::str::next_code_point]
    /// - [A Branchless UTF-8 Decoder](https://nullprogram.com/blog/2017/10/06/)
    /// - [Flexible and Economical UTF-8 Decoder](https://bjoern.hoehrmann.de/utf-8/decoder/dfa/)
    ///
    /// TODO: SIMD speedup
    #[rustfmt::skip]
    fn decode(self) -> char {
        type Decoder = fn(*const u8) -> char;

        macro_rules! bits {
            ($ptr:expr, $offset:expr, $bits:expr, $shl:expr) => {
                { (*$ptr.byte_offset($offset) & BIT_MASKS[$bits]) as u32 } << $shl
            };
        }

        const ___: Decoder = |ptr: *const u8| char!(*ptr);
        const U1B: Decoder = |ptr: *const u8| char!(*ptr);
        const U2B: Decoder = |ptr: *const u8| char!(bits!(ptr, 0, 5,  6) | bits!(ptr, 1, 6,  0));
        const U3B: Decoder = |ptr: *const u8| char!(bits!(ptr, 0, 4, 12) | bits!(ptr, 1, 6,  6) | bits!(ptr, 2, 6, 0));
        const U4B: Decoder = |ptr: *const u8| char!(bits!(ptr, 0, 3, 18) | bits!(ptr, 1, 6, 12) | bits!(ptr, 2, 6, 6) | bits!(ptr, 3, 6, 0));

        const DECODE_HANDLERS: &[Decoder; 256] = &[
          // 0    1    2    3    4    5    6    7    8    9    A    B    C    D    E    F
            U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, // 0
            U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, // 1
            U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, // 2
            U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, // 3
            U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, // 4
            U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, // 5
            U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, // 6
            U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, U1B, // 7
            ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 8
            ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 9
            ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // A
            ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // B
            ___, ___, U2B, U2B, U2B, U2B, U2B, U2B, U2B, U2B, U2B, U2B, U2B, U2B, U2B, U2B, // C
            U2B, U2B, U2B, U2B, U2B, U2B, U2B, U2B, U2B, U2B, U2B, U2B, U2B, U2B, U2B, U2B, // D
            U3B, U3B, U3B, U3B, U3B, U3B, U3B, U3B, U3B, U3B, U3B, U3B, U3B, U3B, U3B, U3B, // E
            U4B, U4B, U4B, U4B, U4B, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // F
        ];

        unsafe { DECODE_HANDLERS[*self as usize](self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        // TODO: benchmark
        //  jump table branchless code vs conditionals
        assert_eq!("A".as_ptr().decode(), 'A')
    }
}
