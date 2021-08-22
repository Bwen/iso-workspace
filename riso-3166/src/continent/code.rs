use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Code {
    // ENUM START
    AF,
    AS,
    EU,
    NA,
    OC,
    SA,
    AN,
    // ENUM END
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
