use crate::{TryFor, FindFor};

mod alpha2;
pub use alpha2::Alpha2;

mod alpha3;
pub use alpha3::Alpha3;

mod numeric;
pub use numeric::Numeric;

mod data;
use data::COUNTRIES;

#[derive(Debug, Eq, PartialEq)]
pub struct Country {
    pub alpha2: Alpha2,
    pub alpha3: Alpha3,
    pub numeric: Numeric,
    pub name: &'static str,
    pub official_name: &'static str,
}

impl FindFor<Alpha2> for Country {
    fn find_for(value: Alpha2) -> &'static Country {
        COUNTRIES.iter().find(|country| country.alpha2 == value).expect("Infallible")
    }
}

impl FindFor<Alpha3> for Country {
    fn find_for(value: Alpha3) -> &'static Country {
        COUNTRIES.iter().find(|country| country.alpha3 == value).expect("Infallible")
    }
}

impl FindFor<Numeric> for Country {
    fn find_for(value: Numeric) -> &'static Country {
        COUNTRIES.iter().find(|country| country.numeric == value).expect("Infallible")
    }
}

impl TryFor<usize> for Country {
    fn try_for(value: usize) -> Result<&'static Self, &'static str> {
        let number = format!("N{:0>3}", value);
        let result = COUNTRIES.iter().find(|country| country.numeric.to_string() == number);
        if result.is_some() {
            return Ok(result.expect("Infallible"));
        }

        Err("Could not find country for supplied numeric")
    }
}

impl TryFor<&str> for Country {
    fn try_for(value: &str) -> Result<&'static Self, &'static str> {
        if value.len() == 2 {
            let result = COUNTRIES.iter().find(|country| country.alpha2.to_string() == value.to_ascii_uppercase());
            if result.is_some() {
                return Ok(result.expect("Infallible"));
            }

            return Err("Could not find country for supplied alpha-2");
        }

        if value.len() == 3 {
            let result = COUNTRIES.iter().find(|country| country.alpha3.to_string() == value.to_ascii_uppercase());
            if result.is_some() {
                return Ok(result.expect("Infallible"));
            }

            return Err("Could not find country for supplied alpha-3");
        }

        if value.parse::<usize>().is_ok() {
            let numeric = format!("N{:0>3}", value);
            let result = COUNTRIES.iter().find(|country| country.numeric.to_string() == numeric);
            if result.is_some() {
                return Ok(result.expect("Infallible"));
            }

            return Err("Could not find country for supplied numeric");
        }

        let result = COUNTRIES.iter().find(|country| country.name.to_ascii_uppercase() == value.to_ascii_uppercase());
        if result.is_some() {
            return Ok(result.expect("Infallible"));
        }

        Err("Could not find country for supplied name")
    }
}

impl Country {
    pub fn all() -> std::slice::Iter<'static, Country> {
        COUNTRIES.iter()
    }

    pub fn count() -> usize {
        COUNTRIES.iter().count()
    }
}
