use riso_static_traits::TryFor;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use scraper::{Html, Selector};
use workspace_utils::{replace_file_content, update_enum_file};
use riso_3166::country::Country;

const URL_4217_SOURCE: &str = "https://www.six-group.com/dam/download/financial-information/data-center/iso-currrency/amendments/lists/list_one.xml";
const URL_SYMBOLS_SOURCE: &str = "https://www.xe.com/symbols.php";
const FILE_DATA_CURRENCIES: &str = "raw_data/iso-4217.xml";
const FILE_RUST_CODE: &str = "src/currency/code.rs";
const FILE_RUST_NUMERIC: &str = "src/currency/numeric.rs";
const FILE_RUST_DATA_CURRENCIES: &str = "src/currency/data.rs";

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Welcome4 {
    #[serde(rename = "ISO_4217")]
    iso_4217: Iso4217,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Iso4217 {
    #[serde(rename = "CcyTbl")]
    table: Entries,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entries {
    #[serde(rename = "CcyNtry")]
    entires: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    #[serde(rename = "CtryNm")]
    country_name: String,

    #[serde(rename = "CcyNm")]
    name: String,

    #[serde(rename = "Ccy")]
    code: Option<String>,

    #[serde(rename = "CcyNbr")]
    numeric: Option<String>,

    #[serde(rename = "CcyMnrUnts")]
    units: Option<String>,
}

#[derive(Debug)]
pub struct Currency {
    pub code: String,
    pub numeric: String,
    pub name: String,
    pub units: usize,
    pub countries: String,
    pub symbol: String,
}

fn main() {
    let _ = download_data_source();

    let data = fetch_data();
    let codes: Vec<String> = data.iter().map(|(_name, currency)| currency.code.clone()).collect();
    update_enum_file(&codes, FILE_RUST_CODE);

    let numerics: Vec<String> = data.iter().map(|(_name, currency)| format!("N{}", currency.numeric.clone())).collect();
    update_enum_file(&numerics, FILE_RUST_NUMERIC);

    update_data_file(data);
}

fn download_data_source() -> Result<(), ureq::Error> {
    if Path::new(FILE_DATA_CURRENCIES).exists() {
        return Ok(());
    }

    let content = ureq::get(URL_4217_SOURCE).call()?.into_string()?;
    let _ = fs::write(FILE_DATA_CURRENCIES, content);

    Ok(())
}

fn update_data_file(data: HashMap<String, Currency>) {
    println!("Updating {}", FILE_RUST_DATA_CURRENCIES);

    let mut data_strings: Vec<String> = vec![];
    for (_name, currency) in data.iter() {
        let mut numeric = format!("N{}", currency.numeric);
        if numeric == "N" {
            numeric = "None".to_string();
        }

        let mut code = currency.code.to_ascii_uppercase();
        if code.is_empty() {
            code = "None".to_string();
        }

        data_strings.push(format!(
            "    Currency {{ code: Code::{}, numeric: Numeric::{}, name: \"{}\", units: {}, symbol: \"{}\", countries: \"{}\" }},",
            code,
            numeric,
            currency.name,
            currency.units,
            currency.symbol,
            currency.countries,
        ));
    }

    replace_file_content(
        FILE_RUST_DATA_CURRENCIES, 
        r"(?ms).*?// DATA START\n(.*)\s+// DATA END.*", 
        format!("{}\n   ", data_strings.join("\n")).as_str()
    );

    replace_file_content(
        FILE_RUST_DATA_CURRENCIES, 
        r"(static CURRENCIES:\[Currency;[0-9]+\])", 
        format!("static CURRENCIES:[Currency;{}]", data.iter().len()).as_str()
    );
}

fn fetch_symbols() -> HashMap<String, String> {
    let mut symbols: HashMap<String, String> = HashMap::new();
    let page_content = ureq::get(URL_SYMBOLS_SOURCE).call().unwrap().into_string().unwrap();
    let html = Html::parse_fragment(&page_content);
    let selector_table = Selector::parse("table.currencySymblTable tbody tr").unwrap();
    let selector_code = Selector::parse("td:nth-child(2)").unwrap();
    let selector_symbol = Selector::parse("td.cSmbl_Fnt_AU").unwrap();
    
    for row in html.select(&selector_table) {
        let symbol = row.select(&selector_symbol).next();
        let code = row.select(&selector_code).next();
        if let Some(symb) = symbol {
            symbols.entry(code.unwrap().inner_html()).or_insert_with(|| symb.inner_html());
        }
    }

    symbols
}

fn fetch_data() -> HashMap<String, Currency> {
    let symbols = fetch_symbols();
    let mut data: HashMap<String, Currency> = HashMap::new();
    let content = fs::read_to_string(FILE_DATA_CURRENCIES).expect("Unable to read file");
    let model: Iso4217 =  serde_xml_rs::from_str(&*content).unwrap();
    for entry in model.table.entires {
        let country = Country::try_for(entry.country_name.as_str());
        let mut alpha2 = String::from("");
        if country.is_ok() {
            alpha2 = country.expect("Infallible").alpha2.to_string();
        }

        let currency = data.get_mut(&entry.name);
        if let Some(currency_entry) = currency {
            if !alpha2.is_empty() {
                currency_entry.countries = format!("{},{}", currency_entry.countries, alpha2);
            }
        } else {
            let code = entry.code.unwrap_or_else(String::new);
            data.insert(entry.name.clone(), Currency {
                code: code.clone(),
                numeric: entry.numeric.unwrap_or_else(String::new).clone(),
                name: entry.name.clone(),
                units: entry.units.unwrap_or_else(String::new).parse::<usize>().unwrap_or(0),
                countries: alpha2,
                symbol: symbols.get(code.as_str()).unwrap_or(&"".to_string()).clone(),
            });
        }
    }

    data
}
