#![feature(test)]
extern crate test;
use test::Bencher;
use std::str::FromStr;

extern crate riso_3166;
use riso_3166::country::{Country, Alpha2, Alpha3, Numeric};

#[bench]
pub fn alpha2_from_str(b : &mut Bencher) {
    b.iter(|| {
        let _c : Alpha2 = Alpha2::from_str("PH").unwrap();
    });
}

#[bench]
pub fn alpha3_from_str(b : &mut Bencher) {
    b.iter(|| {
        let _c = Alpha3::from_str("PHL").unwrap();
    });
}

#[bench]
pub fn numeric_from_str(b : &mut Bencher) {
    b.iter(|| {
        let _c = Numeric::from_str("N608").unwrap();
    });
}

#[bench]
pub fn country_from_alpha2(b : &mut Bencher) {
    b.iter(|| {
        let c = Country::from(Alpha2::PH);
        let _s = c.name;
        let _s = c.official_name;
    });
}

#[bench]
pub fn country_from_alpha3(b : &mut Bencher) {
    b.iter(|| {
        let c = Country::from(Alpha3::PHL);
        let _s = c.name;
        let _s = c.official_name;
    });
}

#[bench]
pub fn country_from_numeric(b : &mut Bencher) {
    b.iter(|| {
        let c = Country::from(Numeric::N608);
        let _s = c.name;
        let _s = c.official_name;
    });
}