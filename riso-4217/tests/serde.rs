extern crate riso_4217;
use riso_static_traits::{TryFor, FindFor};
use riso_4217::{Currency, Numeric, Code};

#[cfg(test)]
use serde::{Deserialize, Serialize};

#[cfg(test)]
use serde_json::Result;

#[derive(Deserialize)]
struct TestDeserialize {
    text: String,
    currency: Currency,
}

#[derive(Serialize)]
struct TestSerialize {
    text: String,
    currency: Currency,
    numeric: Numeric,
    code: Code,
}

#[test]
fn serde_deserialize() -> Result<()> {
    let json = r#"{"text": "test", "currency": "USD"}"#;
    let test_deserialize: TestDeserialize = serde_json::from_str(json)?;
    assert_eq!(test_deserialize.text, "test");
    assert_eq!(test_deserialize.currency.code, Code::USD);

    Ok(())
}

#[test]
fn serde_serialize() -> Result<()> {
    let currency = Currency::find_for(Code::USD);
    let test_serialize = TestSerialize {
        text: String::from("test"), 
        currency: *currency,
        code: Code::USD,
        numeric: Numeric::N104,
    };

    let test: String = serde_json::to_string(&test_serialize).unwrap();
    assert_eq!(test, "{\"text\":\"test\",\"currency\":{\"code\":\"USD\",\"numeric\":\"840\",\"units\":2,\"symbol\":\"$\",\"countries\":\"AS,BQ,IO,EC,SV,GU,HT,MH,FM,MP,PW,PA,PR,TL,TC,UM,US,VG,VI\"},\"numeric\":\"104\",\"code\":\"USD\"}");

    Ok(())
}