use std::str::FromStr;
use std::convert::TryFrom;

mod alpha2;
pub use alpha2::Alpha2;

mod alpha3;
pub use alpha3::Alpha3;

mod numeric;
pub use numeric::Numeric;

mod data;
use data::COUNTRIES;

#[derive(Debug, Clone, Copy)]
pub struct Country {
    pub alpha2: Alpha2,
    pub alpha3: Alpha3,
    pub numeric: Numeric,
    pub name: &'static str,
    pub official_name: &'static str,
}

impl From<Alpha2> for Country {
    fn from(value: Alpha2) -> Country {
        COUNTRIES.iter().filter(|country| country.alpha2 == value).next().copied().expect("Infallible")
    }
}

impl From<Alpha3> for Country {
    fn from(value: Alpha3) -> Country {
        COUNTRIES.iter().filter(|country| country.alpha3 == value).next().copied().expect("Infallible")
    }
}

impl From<Numeric> for Country {
    fn from(value: Numeric) -> Country {
        COUNTRIES.iter().filter(|country| country.numeric == value).next().copied().expect("Infallible")
    }
}

impl TryFrom<usize> for Country {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let number = format!("N{:0>3}", value);
        let numeric = Numeric::from_str(number.as_str());
        if numeric.is_ok() {
            return Ok(Country::from(numeric.expect("Infallible")));
        }

        Err("Could not find country for supplied numeric")
    }
}

impl TryFrom<&str> for Country {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 2 {
            let result = COUNTRIES.iter().filter(|country| country.alpha2.to_string() == value.to_ascii_uppercase()).next();
            if result.is_some() {
                return Ok(result.copied().expect("Infallible"));
            }

            return Err("Could not find country for supplied alpha-2");
        }

        if value.len() == 3 {
            let result = COUNTRIES.iter().filter(|country| country.alpha3.to_string() == value.to_ascii_uppercase()).next();
            if result.is_some() {
                return Ok(result.copied().expect("Infallible"));
            }

            return Err("Could not find country for supplied alpha-3");
        }

        if value.parse::<usize>().is_ok() {
            let numeric = format!("N{:0>3}", value);
            let result = COUNTRIES.iter().filter(|country| country.numeric.to_string() == numeric).next();
            if result.is_some() {
                return Ok(result.copied().expect("Infallible"));
            }

            return Err("Could not find country for supplied numeric");
        }

        let result = COUNTRIES.iter().filter(|country| country.name.to_ascii_uppercase() == value.to_ascii_uppercase()).next();
        if result.is_some() {
            return Ok(result.copied().expect("Infallible"));
        }

        Err("Could not find country for supplied name")
    }
}

impl Country {
    pub fn all() -> Vec<Self> {
        COUNTRIES.iter().copied().collect()
    }

    pub fn count() -> usize {
        COUNTRIES.iter().count()
    }
}
