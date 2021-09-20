mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Array;

use riso_static_traits::{ FindFor, TryFor, IterFor };
use riso_3166::country::{ Country, Detail };
use riso_3166::subdivision::{ Subdivision };
use riso_3166::continent::Continent;
use riso_4217::Currency;
use riso_639::Language;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Riso3166 {
}

#[wasm_bindgen]
impl Riso3166 {
    pub fn countries() -> Array {
        let countries = Array::new();
        for country in Country::all() {
            countries.push(&JsValue::from_serde(country).unwrap());
        }

        countries
    }

    pub fn countries_for_continent(code: &str) -> Array {
        let countries = Array::new();
        let cont = Continent::try_for(code);
        if let Ok(continent) = cont {
            for detail in Detail::iter_for(*continent) {
                countries.push(&JsValue::from_serde(Country::find_for(detail.alpha2)).unwrap());
            }
        }

        countries
    }

    pub fn country_try_for(alpha2: &str) -> JsValue {
        if let Ok(country) = Country::try_for(alpha2) {
            return JsValue::from_serde(country).unwrap();
        }

        JsValue::NULL
    }

    pub fn country_details_for(alpha2: &str) -> JsValue {
        if let Ok(country) = Country::try_for(alpha2) {
            return JsValue::from_serde(country.details()).unwrap();
        }

        JsValue::NULL
    }

    pub fn continents() -> Array {
        let continents = Array::new();
        for continent in Continent::all() {
            continents.push(&JsValue::from_serde(continent).unwrap());
        }

        continents
    }

    pub fn continent_try_for(code: &str) -> JsValue {
        if let Ok(continent) = Continent::try_for(code) {
            return JsValue::from_serde(continent).unwrap();
        }

        JsValue::NULL
    }

    pub fn subdivisions() -> Array {
        let subdivisions = Array::new();
        for subdivision in Subdivision::all() {
            subdivisions.push(&JsValue::from_serde(subdivision).unwrap());
        }

        subdivisions
    }

    pub fn subdivisions_parents() -> Array {
        let subdivisions = Array::new();
        for subdivision in Subdivision::parents() {
            subdivisions.push(&JsValue::from_serde(subdivision).unwrap());
        }

        subdivisions
    }

    pub fn subdivisions_parents_country(alpha2: &str) -> Array {
        let subdivisions = Array::new();
        for subdivision in Subdivision::parents().iter().filter(|sub| sub.country.to_string() == alpha2.to_ascii_uppercase()) {
            subdivisions.push(&JsValue::from_serde(subdivision).unwrap());
        }

        subdivisions
    }

    pub fn subdivisions_for_parent(code: &str) -> Array {
        let subdivisions = Array::new();
        for subdivision in Subdivision::all().filter(|sub| sub.parent.to_string() == code.to_ascii_uppercase()) {
            subdivisions.push(&JsValue::from_serde(subdivision).unwrap());
        }

        subdivisions
    }

    pub fn subdivision_try_for(code: &str) -> JsValue {
        if let Ok(subdivision) = Subdivision::try_for(code) {
            return JsValue::from_serde(subdivision).unwrap();
        }

        JsValue::NULL
    }
}

#[wasm_bindgen]
pub struct Riso4217 {
}

#[wasm_bindgen]
impl Riso4217 {
    pub fn currencies() -> Array {
        let currencies = Array::new();
        for currency in Currency::all() {
            currencies.push(&JsValue::from_serde(currency).unwrap());
        }

        currencies
    }
}

#[wasm_bindgen]
pub struct Riso639 {
}

#[wasm_bindgen]
impl Riso639 {
    pub fn languages() -> Array {
        let languages = Array::new();
        for language in Language::all() {
            languages.push(&JsValue::from_serde(language).unwrap());
        }

        languages
    }
}
