use std::str::FromStr;
use strum::{IntoEnumIterator};

extern crate riso_3166;
use riso_3166::country::{Country, Numeric, Alpha2, Alpha3};
use riso_3166::subdivision::{Subdivision, Code};
use riso_3166::IterFor;

#[test]
fn sanity_checks() {
    let total = Subdivision::count();
    // We plus 1 because of the Code::None that is added
    assert_eq!(Code::iter().count(), total + 1);
}

#[test]
fn subdivision_from() {
    let subdivision = Subdivision::from(Code::CA_QC);
    assert_eq!(subdivision.alpha2, Alpha2::CA);
    assert_eq!(subdivision.alpha3, Alpha3::CAN);
    assert_eq!(subdivision.numeric, Numeric::N124);
    assert_eq!(subdivision.category, "province");
    assert_eq!(subdivision.name, "Quebec");
    assert_eq!(subdivision.parent, Code::None);
}

#[test]
fn subdivision_iterfor() {
    let subdivisions = Subdivision::iter_for(Alpha2::CA);
    assert_eq!(subdivisions.len(), 13);

    for subdivision in subdivisions {
        if subdivision.code == Code::CA_QC {
            assert_eq!(subdivision.alpha2, Alpha2::CA);
            assert_eq!(subdivision.alpha3, Alpha3::CAN);
            assert_eq!(subdivision.numeric, Numeric::N124);
            assert_eq!(subdivision.category, "province");
            assert_eq!(subdivision.name, "Quebec");
            assert_eq!(subdivision.parent, Code::None);
        }
    }

}
