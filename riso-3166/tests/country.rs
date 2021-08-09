use std::str::FromStr;
use std::convert::TryFrom;
use strum::{IntoEnumIterator};

extern crate riso_3166;
use riso_3166::country::{Country, Numeric, Alpha2, Alpha3};

#[test]
fn sanity_checks() {
    let total = Country::count();
    assert_eq!(Alpha2::iter().count(), total);
    assert_eq!(Alpha3::iter().count(), total);
    assert_eq!(Numeric::iter().count(), total);
}

#[test]
fn country_from() {
    let country = Country::from(Alpha2::PH);
    assert_eq!(country.name, "Philippines (the)");
    assert_eq!(country.official_name, "the Republic of the Philippines");
    assert_eq!(country.alpha2, Alpha2::PH);
    assert_eq!(country.alpha3, Alpha3::PHL);
    assert_eq!(country.numeric, Numeric::N608);

    let country = Country::from(Alpha3::RUS);
    assert_eq!(country.name, "Russian Federation (the)");
    assert_eq!(country.official_name, "the Russian Federation");
    assert_eq!(country.alpha2, Alpha2::RU);
    assert_eq!(country.alpha3, Alpha3::RUS);
    assert_eq!(country.numeric, Numeric::N643);

    let country = Country::from(Numeric::N124);
    assert_eq!(country.name, "Canada");
    assert_eq!(country.official_name, "");
    assert_eq!(country.alpha2, Alpha2::CA);
    assert_eq!(country.alpha3, Alpha3::CAN);
    assert_eq!(country.numeric, Numeric::N124);
}

#[test]
fn country_try_from() {
    let country = Country::try_from("canada").unwrap();
    assert_eq!(country.name, "Canada");
    assert_eq!(country.official_name, "");
    assert_eq!(country.alpha2, Alpha2::CA);
    assert_eq!(country.alpha3, Alpha3::CAN);
    assert_eq!(country.numeric, Numeric::N124);

    let country = Country::try_from("ca").unwrap();
    assert_eq!(country.name, "Canada");
    assert_eq!(country.official_name, "");
    assert_eq!(country.alpha2, Alpha2::CA);
    assert_eq!(country.alpha3, Alpha3::CAN);
    assert_eq!(country.numeric, Numeric::N124);

    let country = Country::try_from("rus").unwrap();
    assert_eq!(country.name, "Russian Federation (the)");
    assert_eq!(country.official_name, "the Russian Federation");
    assert_eq!(country.alpha2, Alpha2::RU);
    assert_eq!(country.alpha3, Alpha3::RUS);
    assert_eq!(country.numeric, Numeric::N643);

    let country = Country::try_from("8").unwrap();
    assert_eq!(country.name, "Albania");
    assert_eq!(country.official_name, "the Republic of Albania");
    assert_eq!(country.alpha2, Alpha2::AL);
    assert_eq!(country.alpha3, Alpha3::ALB);
    assert_eq!(country.numeric, Numeric::N008);

    let country = Country::try_from(8 as usize).unwrap();
    assert_eq!(country.name, "Albania");
    assert_eq!(country.official_name, "the Republic of Albania");
    assert_eq!(country.alpha2, Alpha2::AL);
    assert_eq!(country.alpha3, Alpha3::ALB);
    assert_eq!(country.numeric, Numeric::N008);
}