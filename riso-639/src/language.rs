use static_traits::{IterFor, TryFor, FindFor};

#[cfg(feature = "country_details")]
use riso_3166::country::Country;

mod alpha2;
pub use alpha2::Alpha2;

mod alpha3;
pub use alpha3::Alpha3;

mod data;
use data::LANGUAGES;

#[derive(Debug, Eq, PartialEq)]
pub struct Language {
    pub alpha2: Alpha2,
    pub alpha3: Alpha3,
    pub text: &'static str,
}

impl TryFor<&str> for Language {
    fn try_for(value: &str) -> Result<&'static Self, &'static str> {
        if value.len() == 2 {
            let result = LANGUAGES.iter().find(|lang| lang.alpha2.to_string() == value.to_ascii_uppercase());
            if result.is_some() {
                return Ok(result.expect("Infallible"));
            }
        }

        if value.len() == 3 {
            let result = LANGUAGES.iter().find(|lang| lang.alpha3.to_string() == value.to_ascii_uppercase());
            if result.is_some() {
                return Ok(result.expect("Infallible"));
            }
        }

        Err("Could not find language for supplied string")
    }
}

impl FindFor<Alpha2> for Language {
    fn find_for(value: Alpha2) -> &'static Self {
        LANGUAGES.iter().find(|lang| lang.alpha2 == value).expect("Infallible")
    }
}

impl FindFor<Alpha3> for Language {
    fn find_for(value: Alpha3) -> &'static Self {
        LANGUAGES.iter().find(|lang| lang.alpha3 == value).expect("Infallible")
    }
}

#[cfg(feature = "country_details")]
impl IterFor<&Country> for Language {
    fn iter_for(value: &Country) -> Vec<&'static Self> {
        let lang_codes: Vec<String> = value.languages()
            .iter()
            .map(|lang| {
                lang.find('-')
                    .map(|idx| &lang[..idx])
                    .unwrap_or(lang)
                    .trim()
                    .to_ascii_uppercase()
            })
            .collect();

        LANGUAGES.iter().filter(|language| lang_codes.contains(&language.alpha2.to_string())).collect()
    }
}
