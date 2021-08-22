use crate::{FindFor, TryFor};
use crate::continent::Continent;

use super::Alpha2;
use super::Tld;
use super::DETAILS;

#[derive(Debug, Eq, PartialEq)]
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

impl Detail {
    pub fn continent(&self) -> &'static Continent {
        Continent::try_for(self.continent).expect("Infallible")
    }
}
