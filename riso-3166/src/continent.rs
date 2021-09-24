use std::fmt;
use riso_static_traits::{TryFor, FindFor};
use serde::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer, SerializeStruct};

mod data;
use data::CONTINENTS;

mod code;
pub use code::Code;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Continent {
    pub code: Code,
    pub name: &'static str,
}

impl fmt::Display for Continent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.code)
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

impl Continent {
    pub fn all() -> std::slice::Iter<'static, Self> {
        CONTINENTS.iter()
    }
}

impl<'de> Deserialize<'de> for Continent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        use serde::de::Error;

        let string = String::deserialize(deserializer)?;
        let result = Continent::try_for(string.as_str());
        if let Ok(continent) = result {
            return Ok(continent.clone());
        }
        
        Err(D::Error::custom("Unexpected element"))?
    }
}

impl Serialize for Continent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Continent", 2)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}
