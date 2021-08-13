use crate::country::{Alpha2, Alpha3, Numeric};
use crate::{TryFor, FindFor, IterFor};

mod code;
pub use code::Code;

mod data;
use data::SUBDIVISIONS;

#[derive(Debug, Eq, PartialEq)]
pub struct Subdivision {
    pub alpha2: Alpha2,
    pub alpha3: Alpha3,
    pub numeric: Numeric,
    pub code: Code,
    pub parent: Code,
    pub name: &'static str,
    pub category: &'static str,
}

impl FindFor<Code> for Subdivision {
    fn find_for(value: Code) -> &'static Self {
        SUBDIVISIONS.iter().filter(|subdivision| subdivision.code == value).next().expect("Infallible")
    }
}

impl IterFor<Alpha2> for Subdivision {
    fn iter_for(value: Alpha2) -> Vec<&'static Self> {
        SUBDIVISIONS.iter().filter(|subdivision| subdivision.alpha2 == value).collect()
    }
}

impl IterFor<Alpha3> for Subdivision {
    fn iter_for(value: Alpha3) -> Vec<&'static Self> {
        SUBDIVISIONS.iter().filter(|subdivision| subdivision.alpha3 == value).collect()
    }
}

impl IterFor<Numeric> for Subdivision {
    fn iter_for(value: Numeric) -> Vec<&'static Self> {
        SUBDIVISIONS.iter().filter(|subdivision| subdivision.numeric == value).collect()
    }
}

impl Subdivision {
    pub fn count() -> usize {
        SUBDIVISIONS.iter().count()
    }
}
