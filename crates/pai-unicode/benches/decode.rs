#![allow(internal_features)]
#![feature(str_internals)]
#![feature(test)]

extern crate test;

use pai_unicode::decode::UTF8Decode;
use test::Bencher;

#[bench]
fn setup(bencher: &mut Bencher) {
    let n = test::black_box(10000);
    bencher.iter(|| {
        (0..n).for_each(|_| {
            test::black_box("💖").as_ptr().decode();
        })
    })
}

#[bench]
fn setup2(bencher: &mut Bencher) {
    let n = test::black_box(10000);
    bencher.iter(|| unsafe {
        (0..n).for_each(|_| {
            core::str::next_code_point(&mut test::black_box("💖").as_bytes().iter());
        })
    });
}
