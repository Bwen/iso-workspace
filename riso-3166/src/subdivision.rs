use serde::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use crate::country::Alpha2;
use riso_static_traits::{IterFor, FindFor, TryFor};

mod code;
pub use code::Code;

mod data;
use data::SUBDIVISIONS;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Subdivision {
    pub country: Alpha2,
    pub code: Code,
    pub parent: Code,
    pub name: &'static str,
    pub category: &'static str,
}

impl FindFor<Code> for Subdivision {
    fn find_for(value: Code) -> &'static Self {
        SUBDIVISIONS.iter().find(|subdivision| subdivision.code == value).expect("Infallible")
    }
}

impl IterFor<Alpha2> for Subdivision {
    fn iter_for(value: Alpha2) -> Vec<&'static Self> {
        SUBDIVISIONS.iter().filter(|subdivision| subdivision.country == value).collect()
    }
}

impl IterFor<Code> for Subdivision {
    fn iter_for(value: Code) -> Vec<&'static Self> {
        SUBDIVISIONS.iter().filter(|subdivision| subdivision.parent == value).collect()
    }
}

impl TryFor<&str> for Subdivision {
    fn try_for(value: &str) -> Result<&'static Self, &'static str> {
        if value.contains('-') && value[3..].len() >= 1 && value[3..].len() < 4 {
            let result = SUBDIVISIONS.iter().find(|subdivision| subdivision.code.to_string() == value.to_ascii_uppercase());
            if result.is_some() {
                return Ok(result.expect("Infallible"));
            }

            return Err("Could not find subdivision for supplied code");
        }

        let result = SUBDIVISIONS.iter().find(|subdivision| subdivision.name.to_ascii_uppercase() == value.to_ascii_uppercase());
        if result.is_some() {
            return Ok(result.expect("Infallible"));
        }

        Err("Could not find subdivision for supplied name")
    }
}

impl Subdivision {
    pub fn all() -> std::slice::Iter<'static, Self> {
        SUBDIVISIONS.iter()
    }

    pub fn parents() -> Vec<&'static Self> {
        let mut parent_codes: Vec<Code> = SUBDIVISIONS.iter()
            .filter(|subdivision| !subdivision.parent.is_none())
            .map(|subdivision| subdivision.parent)
            .collect();

        parent_codes.dedup();

        let parents: Vec<&'static Self> = SUBDIVISIONS.iter()
            .filter(|subdivision| parent_codes.contains(&subdivision.code))
            .collect();

        parents
    }

    pub fn count() -> usize {
        SUBDIVISIONS.iter().count()
    }
}

impl<'de> Deserialize<'de> for Subdivision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        use serde::de::Error;

        let string = String::deserialize(deserializer)?;
        let result = Subdivision::try_for(string.as_str());
        if let Ok(subdivision) = result {
            return Ok(subdivision.clone());
        }
        
        Err(D::Error::custom("Unexpected element"))?
    }
}

impl Serialize for Subdivision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Subdivision", 5)?;
        state.serialize_field("country", &self.country)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("parent", &self.parent)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("category", &self.category)?;
        state.end()
    }
}
