use static_traits::{TryFor, FindFor};

mod alpha2;
pub use alpha2::Alpha2;

mod alpha3;
pub use alpha3::Alpha3;

mod data;
use data::LANGUAGES;

#[derive(Debug, Eq, PartialEq)]
pub struct Language {
    pub alpha2: Alpha2,
    pub alpha3: Alpha3,
    pub text: &'static str,
}

impl TryFor<Alpha2> for Language {
    fn try_for(value: Alpha2) -> Result<&'static Self, &'static str> {
        let result = LANGUAGES.iter().find(|lang| lang.alpha2 == value);
        if result.is_some() {
            return Ok(result.expect("Infallible"));
        }

        Err("Could not find language for supplied alpha2")
    }
}

impl TryFor<&str> for Language {
    fn try_for(value: &str) -> Result<&'static Self, &'static str> {
        if value.len() == 2 {
            let result = LANGUAGES.iter().find(|lang| lang.alpha2.to_string() == value);
            if result.is_some() {
                return Ok(result.expect("Infallible"));
            }
        }

        if value.len() == 3 {
            let result = LANGUAGES.iter().find(|lang| lang.alpha3.to_string() == value);
            if result.is_some() {
                return Ok(result.expect("Infallible"));
            }
        }


        Err("Could not find language for supplied string")
    }
}

impl FindFor<Alpha3> for Language {
    fn find_for(value: Alpha3) -> &'static Self {
        LANGUAGES.iter().find(|lang| lang.alpha3 == value).expect("Infallible")
    }
}
