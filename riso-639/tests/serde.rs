extern crate riso_639;
use riso_static_traits::{TryFor, FindFor};
use riso_639::{Language, Alpha2, Alpha3};

#[cfg(test)]
use serde::{Deserialize, Serialize};

#[cfg(test)]
use serde_json::Result;

#[derive(Deserialize)]
struct TestDeserialize {
    text: String,
    language: Language,
}

#[derive(Serialize)]
struct TestSerialize {
    text: String,
    language: Language,
    alpha2: Alpha2,
    alpha3: Alpha3,
}

#[test]
fn serde_deserialize() -> Result<()> {
    let json = r#"{"text": "test", "language": "EN"}"#;
    let test_deserialize: TestDeserialize = serde_json::from_str(json)?;
    assert_eq!(test_deserialize.language.alpha2, Alpha2::EN);
    assert_eq!(test_deserialize.text, "test");

    Ok(())
}

#[test]
fn serde_serialize() -> Result<()> {
    let lang = Language::find_for(Alpha2::EN);
    let test_serialize = TestSerialize {
        text: String::from("test"), 
        language: *lang,
        alpha2: Alpha2::FR,
        alpha3: Alpha3::ANG,
    };

    let test: String = serde_json::to_string(&test_serialize).unwrap();
    assert_eq!(test, "{\"text\":\"test\",\"language\":{\"alpha2\":\"EN\",\"alpha3\":\"ENG\",\"text\":\"English\"},\"alpha2\":\"FR\",\"alpha3\":\"ANG\"}");

    Ok(())
}
