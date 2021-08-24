use std::fmt;
use static_traits::{TryFor, FindFor};

mod data;
use data::CONTINENTS;

mod code;
pub use code::Code;

pub struct Continent {
    pub code: Code,
    pub name: &'static str,
}

impl fmt::Display for Continent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

impl FindFor<Code> for Continent {
    fn find_for(value: Code) -> &'static Self {
        CONTINENTS.iter().find(|continent| continent.code == value).expect("Infallible")
    }
}

impl TryFor<&str> for Continent {
    fn try_for(value: &str) -> Result<&'static Self, &'static str> {
        let result = CONTINENTS.iter().find(|continent| value == format!("{:?}", continent.code).as_str());
        if result.is_some() {
            return Ok(result.expect("Infallible"));
        }

        Err("Could not find continent for supplied string")
    }
}
