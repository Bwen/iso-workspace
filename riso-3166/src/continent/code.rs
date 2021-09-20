use std::fmt;
use serde::ser::{Serialize, Serializer};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

impl Serialize for Code {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{}", self).as_str())
    }
}
