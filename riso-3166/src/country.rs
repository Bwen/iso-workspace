use static_traits::{TryFor, FindFor};

mod alpha2;
pub use alpha2::Alpha2;

mod alpha3;
pub use alpha3::Alpha3;

mod numeric;
pub use numeric::Numeric;

mod data;
use data::COUNTRIES;

#[cfg(feature = "country_details")]
mod data_details;
#[cfg(feature = "country_details")]
use data_details::DETAILS;

#[cfg(feature = "country_details")]
mod detail;
#[cfg(feature = "country_details")]
pub use detail::Detail;

#[cfg(feature = "country_details")]
mod tld;
#[cfg(feature = "country_details")]
pub use tld::Tld;

#[cfg(feature = "country_details")]
use crate::continent::Continent;

#[derive(Debug, Eq, PartialEq)]
pub struct Country {
    pub alpha2: Alpha2,
    pub alpha3: Alpha3,
    pub numeric: Numeric,
    pub name: &'static str,
    pub official_name: &'static str,
}

impl FindFor<Alpha2> for Country {
    fn find_for(value: Alpha2) -> &'static Self {
        COUNTRIES.iter().find(|country| country.alpha2 == value).expect("Infallible")
    }
}

impl FindFor<Alpha3> for Country {
    fn find_for(value: Alpha3) -> &'static Self {
        COUNTRIES.iter().find(|country| country.alpha3 == value).expect("Infallible")
    }
}

impl FindFor<Numeric> for Country {
    fn find_for(value: Numeric) -> &'static Self {
        COUNTRIES.iter().find(|country| country.numeric == value).expect("Infallible")
    }
}

impl TryFor<usize> for Country {
    fn try_for(value: usize) -> Result<&'static Self, &'static str> {
        let number = format!("{:0>3}", value);
        let result = COUNTRIES.iter().find(|country| country.numeric.to_string() == number);
        if result.is_some() {
            return Ok(result.expect("Infallible"));
        }

        Err("Could not find country for supplied numeric")
    }
}

impl TryFor<&str> for Country {
    fn try_for(value: &str) -> Result<&'static Self, &'static str> {
        if value.parse::<usize>().is_ok() {
            let numeric = format!("{:0>3}", value);
            let result = COUNTRIES.iter().find(|country| country.numeric.to_string() == numeric);
            if result.is_some() {
                return Ok(result.expect("Infallible"));
            }

            return Err("Could not find country for supplied numeric");
        }

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

        let result = COUNTRIES.iter().find(|country| country.name.to_ascii_uppercase() == value.to_ascii_uppercase());
        if result.is_some() {
            return Ok(result.expect("Infallible"));
        }

        Err("Could not find country for supplied name")
    }
}

impl Country {
    pub fn all() -> std::slice::Iter<'static, Self> {
        COUNTRIES.iter()
    }

    pub fn count() -> usize {
        COUNTRIES.iter().count()
    }

    #[cfg(feature = "country_details")]
    pub fn continent(&self) -> &'static Continent {
        self.details().continent()
    }

    #[cfg(feature = "country_details")]
    pub fn neightboors(&self) -> Vec<&'static Self> {
        let alpha2s: Vec<&str> = self.details().neightboors.split(',').collect();
        COUNTRIES.iter().filter(|country| alpha2s.contains(&format!("{:?}", country.alpha2).as_str())).collect()
    }

    #[cfg(feature = "country_details")]
    pub fn iter_for_population_greater(amount: usize) -> Vec<&'static Self> {
        let alpha2s: Vec<&Alpha2> = DETAILS.iter().filter(|detail| detail.population > amount).map(|c| &c.alpha2).collect();
        COUNTRIES.iter().filter(|country| alpha2s.contains(&&country.alpha2)).collect()
    }

    #[cfg(feature = "country_details")]
    pub fn details(&self) -> &'static Detail {
        DETAILS.iter().find(|detail| detail.alpha2 == self.alpha2).expect("Infallible")
    }

    #[cfg(feature = "country_details")]
    pub fn languages(&self) -> Vec<String> {
        self.details()
            .languages
            .split(',')
            .map(|code| code.to_string())
            .collect::<Vec<String>>()
    }

}
