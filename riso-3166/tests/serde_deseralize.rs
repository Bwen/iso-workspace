use std::str::FromStr;
use std::convert::TryFrom;
use strum::{IntoEnumIterator};
use serde::{Serialize, Deserialize};
use serde_json;

extern crate riso_3166;
use riso_3166::country::{Country, Numeric, Alpha2, Alpha3};

#[derive(Deserialize)]
struct Test1 {
    alpha2: Alpha2,
    alpha3: Alpha3,
    numeric: Numeric,
}

#[test]
fn test_serde_json() {
    let json = r#"
    {
        "alpha2": "CA",
        "alpha3": "CAN",
        "numeric": "124"
    }
    "#;

    let result: Test1 = serde_json::from_str(&json).unwrap();
    assert_eq!(result.alpha2, Alpha2::CA);
    assert_eq!(result.alpha3, Alpha3::CAN);
    assert_eq!(result.numeric, Numeric::N124);
}