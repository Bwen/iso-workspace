#![feature(test)]
extern crate test;
use test::Bencher;

extern crate riso_3166;

use riso_3166::{IterFor, FindFor};
use riso_3166::subdivision::{Subdivision, Code};
use riso_3166::country::Alpha2;

#[bench]
pub fn subdivision_from_code(b : &mut Bencher) {
    b.iter(|| {
        let _c = Subdivision::find_for(Code::CA_QC);
    });
}

#[bench]
pub fn subdivision_iter_for(b : &mut Bencher) {
    b.iter(|| {
        let _c = Subdivision::iter_for(Alpha2::CA);
    });
}
