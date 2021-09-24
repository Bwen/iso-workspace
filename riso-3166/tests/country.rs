extern crate riso_3166;
use riso_3166::country::{Country, Numeric, Alpha2, Alpha3};
use riso_static_traits::{TryFor, FindFor};

#[test]
fn sanity_checks() {
    let total = Country::count();
    assert_eq!(Alpha2::count(), total);
    assert_eq!(Alpha3::count(), total);
    assert_eq!(Numeric::count(), total);
}

#[test]
fn transitional_map_alpha2() {
    let country = Country::try_for("AN").unwrap();
    assert_eq!(country.alpha2, Alpha2::NL);
}

#[test]
fn transitional_map_alpha3() {
    let country = Country::try_for("ANT").unwrap();
    assert_eq!(country.alpha3, Alpha3::NLD);
}

#[test]
fn transitional_map_numeric() {
    let country = Country::try_for("530");
    assert!(country.is_ok());
    assert_eq!(country.unwrap().numeric, Numeric::N528);
}

#[test]
fn country_flags() {
    let countries = Country::all();
    for country in countries {
        assert!(!country.unicode_flag().is_empty());
    }
}

#[test]
fn country_flag() {
    let country = Country::try_for("canada").unwrap();
    assert_eq!(country.unicode_flag(), "🇨🇦");
}

#[test]
fn country_find_for() {
    let country = Country::find_for(Alpha2::PH);
    assert_eq!(country.name, "Philippines (the)");
    assert_eq!(country.official_name, "the Republic of the Philippines");
    assert_eq!(country.alpha2, Alpha2::PH);
    assert_eq!(country.alpha3, Alpha3::PHL);
    assert_eq!(country.numeric, Numeric::N608);

    let country = Country::find_for(Alpha3::RUS);
    assert_eq!(country.name, "Russian Federation (the)");
    assert_eq!(country.official_name, "the Russian Federation");
    assert_eq!(country.alpha2, Alpha2::RU);
    assert_eq!(country.alpha3, Alpha3::RUS);
    assert_eq!(country.numeric, Numeric::N643);

    let country = Country::find_for(Numeric::N124);
    assert_eq!(country.name, "Canada");
    assert_eq!(country.official_name, "");
    assert_eq!(country.alpha2, Alpha2::CA);
    assert_eq!(country.alpha3, Alpha3::CAN);
    assert_eq!(country.numeric, Numeric::N124);
}

#[test]
fn country_try_for() {
    let country = Country::try_for("canada").unwrap();
    assert_eq!(country.name, "Canada");
    assert_eq!(country.official_name, "");
    assert_eq!(country.alpha2, Alpha2::CA);
    assert_eq!(country.alpha3, Alpha3::CAN);
    assert_eq!(country.numeric, Numeric::N124);

    let country = Country::try_for("ca").unwrap();
    assert_eq!(country.name, "Canada");
    assert_eq!(country.official_name, "");
    assert_eq!(country.alpha2, Alpha2::CA);
    assert_eq!(country.alpha3, Alpha3::CAN);
    assert_eq!(country.numeric, Numeric::N124);

    let country = Country::try_for("rus").unwrap();
    assert_eq!(country.name, "Russian Federation (the)");
    assert_eq!(country.official_name, "the Russian Federation");
    assert_eq!(country.alpha2, Alpha2::RU);
    assert_eq!(country.alpha3, Alpha3::RUS);
    assert_eq!(country.numeric, Numeric::N643);

    let country = Country::try_for("40").unwrap();
    assert_eq!(country.name, "Austria");
    assert_eq!(country.official_name, "the Republic of Austria");
    assert_eq!(country.alpha2, Alpha2::AT);
    assert_eq!(country.alpha3, Alpha3::AUT);
    assert_eq!(country.numeric, Numeric::N040);

    let country = Country::try_for(124 as usize).unwrap();
    assert_eq!(country.name, "Canada");
    assert_eq!(country.official_name, "");
    assert_eq!(country.alpha2, Alpha2::CA);
    assert_eq!(country.alpha3, Alpha3::CAN);
    assert_eq!(country.numeric, Numeric::N124);
}

#[test]
fn country_try_for_error() {
    let country: Result<&Country, &str> = Country::try_for("canada2");
    assert!(country.is_err());

    let country: Result<&Country, &str> = Country::try_for("zz");
    assert!(country.is_err());

    let country: Result<&Country, &str> = Country::try_for("zzz");
    assert!(country.is_err());

    let country: Result<&Country, &str> = Country::try_for("99");
    assert!(country.is_err());

    let country: Result<&Country, &str> = Country::try_for(999 as usize);
    assert!(country.is_err());
}

#[test]
fn alpha2_to_string() {
    assert_eq!("CA", format!("{}", Alpha2::CA));
}

#[test]
fn alpha3_to_string() {
    assert_eq!("CAN", format!("{}", Alpha3::CAN));
}

#[test]
fn numeric_to_string() {
    assert_eq!("008", format!("{}", Numeric::N008));
}
