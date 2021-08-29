use grep_regex::RegexMatcher;
use grep_searcher::sinks::UTF8;
use grep_searcher::{Searcher};
use grep_matcher::{Captures, Matcher};
use workspace_utils::{replace_file_content, update_enum_file};

use std::fs;
use std::path::Path;

const URL_SOURCE: &str = "https://download.geonames.org/export/dump/countryInfo.txt";
const FILE_SOURCE_DETAILS: &str = "raw_data/country-details.csv";
const FILE_RUST_TLD: &str = "src/country/tld.rs";
const FILE_RUST_DATA_DETAILS: &str = "src/country/data_details.rs";

use riso_3166::country::Alpha2;

fn main() {
    let _ = download_data_source();
    update_country_details();
}

fn download_data_source() -> Result<(), ureq::Error> {
    if Path::new(FILE_SOURCE_DETAILS).exists() {
        return Ok(());
    }

    let content = ureq::get(URL_SOURCE).call()?.into_string()?;
    let _ = fs::write(FILE_SOURCE_DETAILS, content);

    Ok(())
}

fn update_country_details() {
    let details = fetch_country_details();

    update_enum_file(&details[3].clone(), FILE_RUST_TLD);
    update_country_details_data_file(details);
}

fn fetch_country_details() -> Vec<Vec<String>> {
    let regex = "(?m)^([A-Z]{2})\t.*?\t.*?\t.*?\t.*?\t.*?\t.*?\t([0-9]+)\t([A-Z]{2,})\t([a-z.]{3,}|)\t([A-Z]{3})\t.*?\t(.*?)\t.*?\t(.*?)\t(.*?)\t.*?\t(.*?)\t";
    let matcher = RegexMatcher::new_line_matcher(regex).unwrap();
    let mut captures = matcher.new_captures().unwrap();

    let mut matches: Vec<Vec<String>> = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    Searcher::new()
        .search_path(
            &matcher,
            FILE_SOURCE_DETAILS,
            UTF8(|_, line| {
                let _ = matcher.captures(line.as_bytes(), &mut captures);

                let mut tld = String::from("None");
                if captures.get(4).is_some() {
                    let raw_tld = line[captures.get(4).unwrap()].to_string();
                    if !raw_tld.is_empty() {
                        tld = raw_tld[1..].to_ascii_uppercase();
                    }
                }
        
                let mut postal_regex = String::from("");
                if captures.get(7).is_some() {
                    postal_regex = line[captures.get(7).unwrap()].to_string()
                }

                let mut languages = String::from("");
                if captures.get(8).is_some() {
                    languages = line[captures.get(8).unwrap()].to_string();
                }

                matches[0].push(line[captures.get(1).unwrap()].to_string()); // Alpha2
                matches[1].push(line[captures.get(2).unwrap()].to_string()); // Population
                matches[2].push(line[captures.get(3).unwrap()].to_string()); // Continent
                matches[3].push(tld); // Tld
                matches[4].push(line[captures.get(5).unwrap()].to_string()); // Currency
                matches[5].push(line[captures.get(6).unwrap()].to_string()); // Phone Prefix
                matches[6].push(postal_regex); // Postal Regex
                matches[7].push(languages); // Languages
                matches[8].push(line[captures.get(9).unwrap()].to_string()); // Neightboors
                Ok(true)
            }),
        )
        .unwrap();

    matches
}

fn update_country_details_data_file(data: Vec<Vec<String>>) {
    println!("Updating {}", FILE_RUST_DATA_DETAILS);
    
    let mut data_strings: Vec<String> = vec![];
    for (index, line) in data[0].iter().enumerate() {
        let mut neightboors = String::from("");
        if !data[8][index].is_empty() {
            let countries = data[8][index]
                .split(',')
                .map(|v| {
                    if Alpha2::transitional_map().contains_key(v) {
                        return <&str>::clone(Alpha2::transitional_map().get(v).expect("Infallible"));
                    }
                    v
                })
                .collect::<Vec<&str>>();

            neightboors = countries.join(",");
        }

        let mut alpha2 = line.clone();
        if Alpha2::transitional_map().contains_key(alpha2.as_str()) {
            alpha2 = Alpha2::transitional_map().get(alpha2.as_str()).expect("Infallible").to_string();
        }

        data_strings.push(format!(
            "    Detail {{ alpha2: Alpha2::{}, tld: Tld::{}, population: {}, phone_prefix: \"{}\", postal_regex: r\"{}\", continent: \"{}\", currency: \"{}\", languages: \"{}\", neightboors: \"{}\" }},",
            alpha2,
            data[3][index],
            data[1][index],
            data[5][index],
            data[6][index],
            data[2][index],
            data[4][index],
            data[7][index],
            neightboors,
        ));
    }

    replace_file_content(
        FILE_RUST_DATA_DETAILS, 
        r"(?ms).*?// DATA START\n(.*)\s+// DATA END.*", 
        format!("{}\n   ", data_strings.join("\n")).as_str()
    );

    replace_file_content(
        FILE_RUST_DATA_DETAILS, 
        r"(static DETAILS:\[Detail;[0-9]+\])", 
        format!("static DETAILS:[Detail;{}]", data[0].iter().len()).as_str()
    );
}
