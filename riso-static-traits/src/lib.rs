
pub trait IterFor<T>: Sized {
    fn iter_for(_: T) -> Vec<&'static Self>;
}

pub trait TryFor<T>: Sized {
    fn try_for(_: T) -> Result<&'static Self, &'static str>;
}

pub trait FindFor<T>: Sized {
    fn find_for(_: T) -> &'static Self;
}
