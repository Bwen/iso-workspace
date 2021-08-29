use static_traits::{TryFor, FindFor, IterFor};
use riso_3166::country::{Country, Alpha2};

mod code;
pub use code::Code;

mod numeric;
pub use numeric::Numeric;

mod data;
use data::CURRENCIES;

#[derive(Debug, Eq, PartialEq)]
pub struct Currency {
    pub code: Code,
    pub numeric: Numeric,
    pub name: &'static str,
    pub units: usize,
    pub symbol: &'static str,
    countries: &'static str,
}

impl FindFor<Code> for Currency {
    fn find_for(value: Code) -> &'static Self {
        CURRENCIES.iter().find(|currency| currency.code == value).expect("Infallible")
    }
}

impl FindFor<Numeric> for Currency {
    fn find_for(value: Numeric) -> &'static Self {
        CURRENCIES.iter().find(|currency| currency.numeric == value).expect("Infallible")
    }
}

impl TryFor<usize> for Currency {
    fn try_for(value: usize) -> Result<&'static Self, &'static str> {
        let number = format!("{:0>3}", value);
        let result = CURRENCIES.iter().find(|currency| currency.numeric.to_string() == number);
        if result.is_some() {
            return Ok(result.expect("Infallible"));
        }

        Err("Could not find currency for supplied numeric")
    }
}

impl TryFor<&str> for Currency {
    fn try_for(value: &str) -> Result<&'static Self, &'static str> {
        if value.parse::<usize>().is_ok() {
            let numeric = format!("{:0>3}", value);
            let result = CURRENCIES.iter().find(|currency| currency.numeric.to_string() == numeric);
            if result.is_some() {
                return Ok(result.expect("Infallible"));
            }

            return Err("Could not find currency for supplied numeric");
        }

        if value.len() == 3 {
            let result = CURRENCIES.iter().find(|currency| currency.code.to_string() == value.to_ascii_uppercase());
            if result.is_some() {
                return Ok(result.expect("Infallible"));
            }

            return Err("Could not find currency for supplied code");
        }

        let result = CURRENCIES.iter().find(|currency| currency.name.to_ascii_uppercase() == value.to_ascii_uppercase());
        if result.is_some() {
            return Ok(result.expect("Infallible"));
        }

        Err("Could not find currency for supplied name")
    }
}

impl IterFor<Alpha2> for Currency {
    fn iter_for(value: Alpha2) -> Vec<&'static Self> {
        CURRENCIES.iter().filter(|currency| currency.countries.contains(&value.to_string())).collect()
    }
}

impl Currency {
    pub fn countries(&self) -> Vec<&'static Country> {
        let alpha2s = self.countries.split(',');
        let mut countries: Vec<&Country> = vec![];
        for alpha2 in alpha2s {
            let country = Country::try_for(alpha2);
            if country.is_ok() {
                countries.push(country.expect("Infallible"));
            }
        }

        countries
    }
}
