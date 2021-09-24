extern crate riso_3166;
use riso_static_traits::{TryFor, FindFor};
use riso_3166::country::{Country, Numeric, Alpha2, Alpha3};

#[cfg(test)]
use serde::{Deserialize, Serialize};

#[cfg(test)]
use serde_json::Result;

#[derive(Debug, Deserialize)]
struct TestDeserialize {
    text: String,
    country: Country,
}

#[derive(Serialize)]
struct TestSerialize {
    text: String,
    country: Country,
    numeric: Numeric,
    alpha2: Alpha2,
    alpha3: Alpha3,
}

#[test]
fn serde_deserialize() -> Result<()> {
    let json = r#"{"text": "test", "country": "US"}"#;
    let test_deserialize: TestDeserialize = serde_json::from_str(json)?;
    assert_eq!(test_deserialize.text, "test");
    assert_eq!(test_deserialize.country.alpha3, Alpha3::USA);
    assert_eq!(test_deserialize.country.numeric, Numeric::N840);
    assert_eq!(test_deserialize.country.name, "United States of America (the)");
    assert_eq!(test_deserialize.country.official_name, "the United States of America");

    Ok(())
}

#[test]
fn serde_deserialize_error() -> Result<()> {
    let json = r#"{"text": "test", "country": "USDSF"}"#;
    let result: Result<Country> = serde_json::from_str(json);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn serde_serialize() -> Result<()> {
    let country = Country::find_for(Alpha2::US);
    let test_serialize = TestSerialize {
        text: String::from("test"), 
        country: *country,
        alpha2: Alpha2::US,
        alpha3: Alpha3::USA,
        numeric: Numeric::N104,
    };

    let test: String = serde_json::to_string(&test_serialize).unwrap();
    assert_eq!(test, "{\"text\":\"test\",\"country\":{\"alpha2\":\"US\",\"alpha3\":\"USA\",\"numeric\":\"840\",\"name\":\"United States of America (the)\",\"official_name\":\"the United States of America\",\"flag\":\"🇺🇸\"},\"numeric\":\"104\",\"alpha2\":\"US\",\"alpha3\":\"USA\"}");

    Ok(())
}
