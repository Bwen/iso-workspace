
extern crate riso_639;
use static_traits::{IterFor, TryFor, FindFor};
use riso_639::{Language, Alpha2, Alpha3};

#[test]
fn try_for() {
    let language = Language::try_for("en").unwrap();
    assert_eq!(language.alpha2, Alpha2::EN);
    assert_eq!(language.alpha3, Alpha3::ENG);
    assert_eq!(language.text, "English");

    let language = Language::try_for("awa").unwrap();
    assert_eq!(language.alpha2, Alpha2::None);
    assert_eq!(language.alpha3, Alpha3::AWA);
    assert_eq!(language.text, "Awadhi");
}

#[test]
fn find_for() {
    let language = Language::find_for(Alpha2::LI);
    assert_eq!(language.alpha2, Alpha2::LI);
    assert_eq!(language.alpha3, Alpha3::LIM);
    assert_eq!(language.text, "Limburgan; Limburger; Limburgish");

    //{alpha2: Alpha2::None, alpha3: Alpha3::PEO, text: "Persian, Old (ca.600-400 B.C.)"}
    let language = Language::find_for(Alpha3::PEO);
    assert_eq!(language.alpha2, Alpha2::None);
    assert_eq!(language.alpha3, Alpha3::PEO);
    assert_eq!(language.text, "Persian, Old (ca.600-400 B.C.)");
}
