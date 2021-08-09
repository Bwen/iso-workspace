use crate::country::{Alpha2, Alpha3, Numeric};
use crate::IterFor;

mod code;
pub use code::Code;

mod data;
use data::SUBDIVISIONS;

#[derive(Debug, Clone, Copy)]
pub struct Subdivision {
    pub alpha2: Alpha2,
    pub alpha3: Alpha3,
    pub numeric: Numeric,
    pub code: Code,
    pub parent: Code,
    pub name: &'static str,
    pub category: &'static str,
}

impl From<Code> for Subdivision {
    fn from(value: Code) -> Self {
        SUBDIVISIONS.iter().filter(|subdivision| subdivision.code == value).next().copied().expect("Infallible")
    }
}

impl IterFor<Alpha2> for Subdivision {
    fn iter_for(value: Alpha2) -> Vec<Self> {
        SUBDIVISIONS.iter().filter(|subdivision| subdivision.alpha2 == value).copied().collect()
    }
}

impl IterFor<Alpha3> for Subdivision {
    fn iter_for(value: Alpha3) -> Vec<Self> {
        SUBDIVISIONS.iter().filter(|subdivision| subdivision.alpha3 == value).copied().collect()
    }
}

impl IterFor<Numeric> for Subdivision {
    fn iter_for(value: Numeric) -> Vec<Self> {
        SUBDIVISIONS.iter().filter(|subdivision| subdivision.numeric == value).copied().collect()
    }
}

impl Subdivision {
    pub fn count() -> usize {
        SUBDIVISIONS.iter().count()
    }
}
