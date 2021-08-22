use grep_matcher;
use grep_regex::RegexMatcher;
use grep_searcher::sinks::UTF8;
use grep_searcher::{Searcher, SearcherBuilder};
use grep_matcher::{Captures, Matcher};
// use ureq;
use std::fs;
use std::fs::File;
use std::io::Error;
use std::path::Path;

const ISO_COUNTRY_DATA_FILE: &str = "raw_data/countryInfo.txt";
const ISO_TLD_FILE: &str = "src/country/tld.rs";
const ISO_DETAILS_RUST_FILE: &str = "src/country/data_details.rs";

use riso_3166::country::Alpha2;
use std::collections::HashMap;

fn main() {
    update_country_details();
}

fn update_country_details() {
    let details = fetch_country_details();

    update_enum_file(details[3].clone(), ISO_TLD_FILE);
    update_country_details_data_file(details);
}

fn update_country_details_data_file(data: Vec<Vec<String>>) {
    println!("Updating {}", ISO_DETAILS_RUST_FILE);
    
    let mut data_strings: Vec<String> = vec![];
    for (index, line) in data[0].iter().enumerate() {
        let mut neightboors = String::from("");
        if data[8][index] != "" {
            let countries = data[8][index]
                .split(",")
                .map(|v| {
                    if Alpha2::transitional_map().contains_key(v) {
                        return Alpha2::transitional_map().get(v).expect("Infallible").clone();
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

    // Update the data
    let file: String = fs::read_to_string(ISO_DETAILS_RUST_FILE).unwrap();
    let matcher = RegexMatcher::new(r"(?ms).*?// DATA START\n(.*)\s+// DATA END.*").expect("Invalid Regex");
    let mut captures = matcher.new_captures().unwrap();

    let mut new_file: String = String::from("");
    SearcherBuilder::new()
        .multi_line(true)
        .build()
        .search_slice(
            &matcher,
            &file.as_bytes(),
            UTF8(|_, result| {
                matcher.captures(result.as_bytes(), &mut captures);
                new_file = file.replace(
                    &result[captures.get(1).unwrap()].to_string(),
                    format!("{}\n   ", data_strings.join("\n")).as_str(),
                );
                Ok(true)
            }),
        )
        .unwrap();

    // Update the size of the static array
    let file: String = new_file.clone();
    let matcher = RegexMatcher::new(r"(static DETAILS:\[Detail;[0-9]+\])").expect("Invalid Regex");
    let mut captures = matcher.new_captures().unwrap();

    let mut new_file: String = String::from("");
    SearcherBuilder::new()
        .multi_line(true)
        .build()
        .search_slice(
            &matcher,
            &file.as_bytes(),
            UTF8(|_, result| {
                matcher.captures(result.as_bytes(), &mut captures);
                new_file = file.replace(
                    &result[captures.get(1).unwrap()].to_string(),
                    format!("static DETAILS:[Detail;{}]", data[0].iter().len()).as_str(),
                );
                Ok(true)
            }),
        )
        .unwrap();

    fs::write(ISO_DETAILS_RUST_FILE, new_file);
}

fn fetch_country_details() -> Vec<Vec<String>> {
    let regex = "(?m)^([A-Z]{2})\t.*?\t.*?\t.*?\t.*?\t.*?\t.*?\t([0-9]+)\t([A-Z]{2,})\t([a-z­.]{3,}|)\t([A-Z]{3})\t.*?\t(.*?)\t.*?\t(.*?)\t(.*?)\t.*?\t(.*?)\t";
    let matcher = RegexMatcher::new_line_matcher(regex).unwrap();
    let mut captures = matcher.new_captures().unwrap();

    let mut matches: Vec<Vec<String>> = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    Searcher::new()
        .search_path(
            &matcher,
            ISO_COUNTRY_DATA_FILE,
            UTF8(|_, line| {
                matcher.captures(line.as_bytes(), &mut captures);

                let mut tld = String::from("None");
                if captures.get(4).is_some() {
                    let raw_tld = line[captures.get(4).unwrap()].to_string();
                    if raw_tld != "" {
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

fn update_enum_file(mut data: Vec<String>, file_path: &str) {
    println!("Updating {}", file_path);

    let mut enums: Vec<&str> = data.iter()
        .filter(|v| v.as_str() != "None")
        .map(|v| v.trim())
        .collect();

    enums.sort();
    enums.dedup();

    let mut codes_enum: String = format!("    {}", enums.join(",\n    "));
    let file: String = fs::read_to_string(file_path).unwrap();
    let matcher = RegexMatcher::new(r"(?ms).*?// ENUM START\n(.*)\s+// ENUM END.*").expect("Invalid Regex");
    let mut captures = matcher.new_captures().unwrap();

    let mut new_file: String = String::from("");
    SearcherBuilder::new()
        .multi_line(true)
        .build()
        .search_slice(
            &matcher,
            &file.as_bytes(),
            UTF8(|_, result| {
                matcher.captures(result.as_bytes(), &mut captures);
                new_file = file.replace(
                    &result[captures.get(1).unwrap()].to_string(),
                    format!("{},\n   ", &codes_enum).as_str(),
                );

                Ok(true)
            }),
        )
        .unwrap();

    fs::write(file_path, new_file);
}
