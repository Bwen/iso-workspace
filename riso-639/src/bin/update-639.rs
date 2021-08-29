use std::collections::HashMap;
use std::fs;
use std::path::Path;
use workspace_utils::{match_lines, replace_file_content, update_enum_file};

const URL_SOURCE: &str = "https://www.loc.gov/standards/iso639-2/ISO-639-2_utf-8.txt";
const FILE_DATA_LANGUAGES: &str = "raw_data/iso-639.csv";
const FILE_RUST_ALPHA2: &str = "src/language/alpha2.rs";
const FILE_RUST_ALPHA3: &str = "src/language/alpha3.rs";
const FILE_RUST_DATA_LANGUAGES: &str = "src/language/data.rs";

fn main() {
    let _ = download_data_source();

    let data: HashMap<&str, Vec<String>> = match_lines(
        FILE_DATA_LANGUAGES,
        "(?m)^([a-z]{3})\\|.*?\\|([a-z]{2}|)\\|(.*?)\\|",
        vec!["alpha3", "alpha2", "text"]
    );

    update_enum_file(&data.get("alpha2").unwrap().clone(), FILE_RUST_ALPHA2);
    update_enum_file(&data.get("alpha3").unwrap().clone(), FILE_RUST_ALPHA3);
    update_data_file(data);
}

fn download_data_source() -> Result<(), ureq::Error> {
    if Path::new(FILE_DATA_LANGUAGES).exists() {
        return Ok(());
    }

    let content = ureq::get(URL_SOURCE).call()?.into_string()?;
    let _ = fs::write(FILE_DATA_LANGUAGES, content);

    Ok(())
}

fn update_data_file(data: HashMap<&str, Vec<String>>) {
    println!("Updating {}", FILE_RUST_DATA_LANGUAGES);

    let mut data_strings: Vec<String> = vec![];
    for (index, alpha3) in data.get("alpha3").unwrap().iter().enumerate() {
        let mut alpha2 = data.get("alpha2").unwrap()[index].to_ascii_uppercase();
        if alpha2.is_empty() {
            alpha2 = "None".to_string();
        }

        data_strings.push(format!(
            "    Language {{alpha2: Alpha2::{}, alpha3: Alpha3::{}, text: \"{}\"}},",
            alpha2,
            alpha3.to_ascii_uppercase(),
            data.get("text").unwrap()[index],
        ));
    }

    replace_file_content(
        FILE_RUST_DATA_LANGUAGES, 
        r"(?ms).*?// DATA START\n(.*)\s+// DATA END.*", 
        format!("{}\n   ", data_strings.join("\n")).as_str()
    );

    replace_file_content(
        FILE_RUST_DATA_LANGUAGES, 
        r"(static LANGUAGES:\[Language;[0-9]+\])", 
        format!("static LANGUAGES:[Language;{}]", data.get("alpha2").unwrap().iter().len()).as_str()
    );
}
