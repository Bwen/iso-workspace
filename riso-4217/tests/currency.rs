extern crate riso_4217;
use riso_4217::{Currency, Numeric, Code};
use riso_3166::country::{Country, Alpha2};
use riso_static_traits::{TryFor, FindFor, IterFor};

#[test]
fn try_for() {
    let currency = Currency::try_for("saint helena pound").unwrap();
    assert_eq!(currency.code, Code::SHP);
    assert_eq!(currency.numeric, Numeric::N654);
    assert_eq!(currency.name, "Saint Helena Pound");
    assert_eq!(currency.units, 2);
    let countries = currency.countries();
    assert_eq!(countries.len(), 1);
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::SH));

    let currency = Currency::try_for("xof").unwrap();
    assert_eq!(currency.code, Code::XOF);
    assert_eq!(currency.numeric, Numeric::N952);
    assert_eq!(currency.name, "CFA Franc BCEAO");
    assert_eq!(currency.units, 0);
    let countries = currency.countries();
    assert_eq!(countries.len(), 7);
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::BJ));
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::BF));
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::GW));
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::ML));
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::NE));
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::SN));
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::TG));

    let currency = Currency::try_for("604").unwrap();
    assert_eq!(currency.code, Code::PEN);
    assert_eq!(currency.numeric, Numeric::N604);
    assert_eq!(currency.name, "Sol");
    assert_eq!(currency.units, 2);
    let countries = currency.countries();
    assert_eq!(countries.len(), 1);
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::PE));
}

#[test]
fn find_for() {
    let currency = Currency::find_for(Numeric::N654);
    assert_eq!(currency.code, Code::SHP);
    assert_eq!(currency.numeric, Numeric::N654);
    assert_eq!(currency.name, "Saint Helena Pound");
    assert_eq!(currency.units, 2);
    let countries = currency.countries();
    assert_eq!(countries.len(), 1);
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::SH));

    let currency = Currency::find_for(Code::XOF);
    assert_eq!(currency.code, Code::XOF);
    assert_eq!(currency.numeric, Numeric::N952);
    assert_eq!(currency.name, "CFA Franc BCEAO");
    assert_eq!(currency.units, 0);
    let countries = currency.countries();
    assert_eq!(countries.len(), 7);
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::BJ));
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::BF));
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::GW));
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::ML));
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::NE));
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::SN));
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::TG));

    let currency = Currency::try_for(604_usize).unwrap();
    assert_eq!(currency.code, Code::PEN);
    assert_eq!(currency.numeric, Numeric::N604);
    assert_eq!(currency.name, "Sol");
    assert_eq!(currency.units, 2);
    let countries = currency.countries();
    assert_eq!(countries.len(), 1);
    assert!(countries.iter().any(|c| c.alpha2 == Alpha2::PE));
}

#[test]
fn iter_for() {
    let country = Country::find_for(Alpha2::US);
    let currencies = Currency::iter_for(country);
    assert_eq!(currencies.len(), 2);
    assert!(currencies.iter().any(|c| c.code == Code::USD));
    assert!(currencies.iter().any(|c| c.code == Code::USN));

    let currencies = Currency::iter_for(Alpha2::US);
    assert_eq!(currencies.len(), 2);
    assert!(currencies.iter().any(|c| c.code == Code::USD));
    assert!(currencies.iter().any(|c| c.code == Code::USN));
}