use crate::country::Alpha2;
use static_traits::{TryFor, FindFor};

mod code;
pub use code::Code;

mod data;
use data::SUBDIVISIONS;

#[derive(Debug, Eq, PartialEq)]
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

impl Subdivision {
    pub fn count() -> usize {
        SUBDIVISIONS.iter().count()
    }
}
