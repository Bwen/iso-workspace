use serde::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use riso_static_traits::{TryFor, FindFor, IterFor};
use crate::continent::Continent;

use super::Alpha2;
use super::Tld;
use super::DETAILS;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Detail {
    pub alpha2: Alpha2,
    pub tld: Tld,
    pub population: usize,
    pub phone_prefix: &'static str,
    pub postal_regex: &'static str,
    pub currency: &'static str,
    pub (super) continent: &'static str,
    pub (super) languages: &'static str,
    pub (super) neightboors: &'static str,
}

impl FindFor<Alpha2> for Detail {
    fn find_for(value: Alpha2) -> &'static Self {
        DETAILS.iter().find(|detail| detail.alpha2 == value).expect("Infallible")
    }
}

impl TryFor<&str> for Detail {
    fn try_for(value: &str) -> Result<&'static Self, &'static str> {
        if value.len() == 2 {
            let result = DETAILS.iter().find(|detail| detail.alpha2.to_string() == value.to_ascii_uppercase());
            if result.is_some() {
                return Ok(result.expect("Infallible"));
            }

            return Err("Could not find country details for supplied alpha-2");
        }

        Err("Could not find country details for supplied string")
    }
}

impl IterFor<Continent> for Detail {
    fn iter_for(value: Continent) -> Vec<&'static Self> {
        DETAILS.iter().filter(|detail| detail.continent.to_string() == value.code.to_string()).collect()
    }
}

impl Detail {
    pub fn continent(&self) -> &'static Continent {
        Continent::try_for(self.continent).expect("Infallible")
    }
}

impl<'de> Deserialize<'de> for Detail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        use serde::de::Error;

        let string = String::deserialize(deserializer)?;
        let result = Detail::try_for(string.as_str());
        if let Ok(detail) = result {
            return Ok(detail.clone());
        }
        
        Err(D::Error::custom("Unexpected element"))?
    }
}

impl Serialize for Detail {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Country", 9)?;
        state.serialize_field("alpha2", &self.alpha2)?;
        state.serialize_field("tld", &self.tld)?;
        state.serialize_field("population", &self.population)?;
        state.serialize_field("phone_prefix", &self.phone_prefix)?;
        state.serialize_field("postal_regex", &self.postal_regex)?;
        state.serialize_field("currency", &self.currency)?;
        state.serialize_field("continent", &self.continent)?;
        state.serialize_field("languages", &self.languages)?;
        state.serialize_field("neightboors", &self.neightboors)?;

        state.end()
    }
}
