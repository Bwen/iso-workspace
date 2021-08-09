
pub trait IterFor<T>: Sized {
    fn iter_for(_: T) -> Vec<Self>;
}

pub mod country;
#[cfg(feature = "subdivisions")]
pub mod subdivision;
