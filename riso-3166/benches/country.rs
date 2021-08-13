#![feature(test)]
extern crate test;
use test::Bencher;

extern crate riso_3166;
use riso_3166::FindFor;
use riso_3166::country::{Country, Alpha2, Alpha3, Numeric};

#[bench]
pub fn country_from_alpha2(b : &mut Bencher) {
    b.iter(|| {
        let c = Country::find_for(Alpha2::PH);
        let _s = c.name;
        let _s = c.official_name;
    });
}

#[bench]
pub fn country_from_alpha3(b : &mut Bencher) {
    b.iter(|| {
        let c = Country::find_for(Alpha3::PHL);
        let _s = c.name;
        let _s = c.official_name;
    });
}

#[bench]
pub fn country_from_numeric(b : &mut Bencher) {
    b.iter(|| {
        let c = Country::find_for(Numeric::N608);
        let _s = c.name;
        let _s = c.official_name;
    });
}