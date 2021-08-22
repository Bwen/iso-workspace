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
use std::collections::HashMap;

const ISO_31661_DATA_FILE: &str = "raw_data/country-codes.csv";
const ISO_31661_ALPHA2_FILE: &str = "src/country/alpha2.rs";
const ISO_31661_ALPHA3_FILE: &str = "src/country/alpha3.rs";
const ISO_31661_NUMERIC_FILE: &str = "src/country/numeric.rs";
const ISO_31661_RUST_FILE: &str = "src/country/data.rs";

const ISO_31662_CATEGORIES_DATA_FILE: &str = "raw_data/subdivision-categories.csv";
const ISO_31662_NAME_DATA_FILE: &str = "raw_data/subdivision-names.csv";
const ISO_31662_SUBDIVISIONS_DATA_FILE: &str = "raw_data/subdivisions.csv";
const ISO_31662_SUBDIVISION_RUST_FILE: &str = "src/subdivision/data.rs";
const ISO_31662_CODE_FILE: &str = "src/subdivision/code.rs";

fn main() {
    // update_31661();
    update_31662();
}

fn update_31662() {
    let categories = fetch_31662_categories();
    let names = fetch_31662_names();
    let codes = fetch_31662_codes();
    let mut unique_codes = codes[4].clone();
    unique_codes.dedup();
    // update_enum_file(&unique_codes,ISO_31662_CODE_FILE);
    update_31662_data_file(codes, categories, names);
}

fn update_31662_data_file(data: Vec<Vec<String>>, categories: HashMap<usize, String>, names: HashMap<String, String>) {
    println!("Updating {}", ISO_31662_SUBDIVISION_RUST_FILE);

    let mut data_strings: Vec<String> = vec![];
    for (index, line) in data[0].iter().enumerate() {
        let category_id: usize = data[3][index].parse().unwrap();
        let mut parent_code = "None";
        if data[5][index] != "" {
            parent_code = &data[5][index];
        }

        data_strings.push(format!(
            "    Subdivision {{alpha2: Alpha2::{}, alpha3: Alpha3::{}, numeric: Numeric::N{}, code: Code::{}, parent: Code::{}, name: \"{}\", category: \"{}\"}},",
            line,
            data[1][index],
            data[2][index],
            data[4][index],
            parent_code,
            names.get(&data[4][index]).unwrap(),
            categories.get(&category_id).unwrap(),
        ));
    }

    // Update the data
    let file: String = fs::read_to_string(ISO_31662_SUBDIVISION_RUST_FILE).unwrap();
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
    let matcher = RegexMatcher::new(r"(static SUBDIVISIONS:\[Subdivision;[0-9]+\])").expect("Invalid Regex");
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
                    format!("static SUBDIVISIONS:[Subdivision;{}]", data[0].iter().len()).as_str(),
                );
                Ok(true)
            }),
        )
        .unwrap();

    fs::write(ISO_31662_SUBDIVISION_RUST_FILE, new_file);
}

fn fetch_31662_codes() -> Vec<Vec<String>> {
    let regex = "(?m)^\"([A-Z]{2})\",\"([A-Z]{3})\",\"([0-9]{3})\",\"([0-9]{3})\",\"([A-Z]{2}-.*)\",\".*\",\"(.*)\"";
    let matcher = RegexMatcher::new_line_matcher(regex).unwrap();
    let mut captures = matcher.new_captures().unwrap();

    let mut matches: Vec<Vec<String>> = vec![vec![], vec![], vec![], vec![], vec![], vec![]];
    Searcher::new()
        .search_path(
            &matcher,
            ISO_31662_SUBDIVISIONS_DATA_FILE,
            UTF8(|_, line| {
                matcher.captures(line.as_bytes(), &mut captures);

                let mut parent = String::from("None");
                if captures.get(6).is_some() {
                    let parent_code = line[captures.get(6).unwrap()].to_string();
                    let parent_code_parts: Vec<&str> = parent_code.split("-").collect();
                    parent = parent_code_parts.join("_");
                }

                let code = line[captures.get(5).unwrap()].to_string();
                let code_parts: Vec<&str> = code.split("-").collect();
                matches[0].push(line[captures.get(1).unwrap()].to_string()); // Alpha2
                matches[1].push(line[captures.get(2).unwrap()].to_string()); // Alpha3
                matches[2].push(line[captures.get(3).unwrap()].to_string()); // Numeric
                matches[3].push(line[captures.get(4).unwrap()].to_string()); // Category id
                matches[4].push(code_parts.join("_")); // Code
                matches[5].push(parent); // Parent Code
                Ok(true)
            }),
        )
        .unwrap();

    matches
}

fn strip_comment<'a>(input: &'a str, markers: &[char]) -> &'a str {
    input
        .find(markers)
        .map(|idx| &input[..idx])
        .unwrap_or(input)
        .trim()
}

fn fetch_31662_names() -> HashMap<String, String> {
    let regex = "(?m)^\".*\",\".*\",\".*\",\".*\",\"([A-Z]{2}-.*)\",\".*\",\".*\",\"(.*)\",\".*\",\".*\"";
    let matcher = RegexMatcher::new_line_matcher(regex).unwrap();
    let mut captures = matcher.new_captures().unwrap();

    let mut matches: HashMap<String, String> = HashMap::new();
    Searcher::new()
        .search_path(
            &matcher,
            ISO_31662_NAME_DATA_FILE,
            UTF8(|_, line| {
                matcher.captures(line.as_bytes(), &mut captures);
                let text = &line[captures.get(2).unwrap()];
                let code = line[captures.get(1).unwrap()].to_string();
                let code_parts: Vec<&str> = code.split("-").collect();

                // The file `subdivision-names.csv` provided by `iso.org` is confusing,
                // it does not always provide english text for subdivisions, which is insane...
                // So we take the first entry only, it is not guarantee to be in english. Most are?
                let sanitized_text = strip_comment(text, &['(']);
                matches.entry(code_parts.join("_")).or_insert(sanitized_text.trim().to_string());

                Ok(true)
            }),
        )
        .unwrap();

    matches
}

fn fetch_31662_categories() -> HashMap<usize, String> {
    let regex = "(?m)^\".*\",\".*\",\".*\",\"([0-9]+)\",\"en\",\".*\",\"(.*)\",\".*\"";
    let matcher = RegexMatcher::new_line_matcher(regex).unwrap();
    let mut captures = matcher.new_captures().unwrap();

    let mut matches: HashMap<usize, String> = HashMap::new();
    Searcher::new()
        .search_path(
            &matcher,
            ISO_31662_CATEGORIES_DATA_FILE,
            UTF8(|_, line| {
                matcher.captures(line.as_bytes(), &mut captures);
                let text = &line[captures.get(2).unwrap()];
                let id: usize = line[captures.get(1).unwrap()].parse().unwrap();
                matches.entry(id).or_insert(text.to_string());

                Ok(true)
            }),
        )
        .unwrap();

    matches
}

fn update_31661() {
    let data = fetch_31661_data();
    update_enum_file(&data[0], ISO_31661_ALPHA2_FILE);
    update_enum_file(&data[1], ISO_31661_ALPHA3_FILE);
    update_enum_file(&data[2], ISO_31661_NUMERIC_FILE);
    update_31661_data_file(data);
}

fn fetch_31661_data() -> Vec<Vec<String>> {
    let regex = "(?m)^\"([A-Z]{2})\",\"([A-Z]{3})\",\"([0-9]{1,3})\",\".*\",\"([a-z\\-]+)\",\"(.*)\",\".*\",\"(.*)\"";
    let matcher = RegexMatcher::new_line_matcher(regex).unwrap();
    let mut captures = matcher.new_captures().unwrap();

    let mut matches: Vec<Vec<String>> = vec![vec![], vec![], vec![], vec![], vec![]];
    Searcher::new()
        .search_path(
            &matcher,
            ISO_31661_DATA_FILE,
            UTF8(|_, line| {
                matcher.captures(line.as_bytes(), &mut captures);
                let status = line[captures.get(4).unwrap()].to_string();
                if status != "officially-assigned" {
                    return Ok(true);
                }

                matches[0].push(line[captures.get(1).unwrap()].to_string()); // Alpha2
                matches[1].push(line[captures.get(2).unwrap()].to_string()); // Alpha3
                matches[2].push(line[captures.get(3).unwrap()].to_string()); // Numeric
                matches[3].push(line[captures.get(5).unwrap()].to_string()); // Short Name
                matches[4].push(line[captures.get(6).unwrap()].to_string()); // Full Name

                Ok(true)
            }),
        )
        .unwrap();

    matches
}

fn update_31661_data_file(data: Vec<Vec<String>>) {
    println!("Updating {}", ISO_31661_RUST_FILE);

    let mut data_strings: Vec<String> = vec![];
    for (index, line) in data[0].iter().enumerate() {
        data_strings.push(format!(
            "    Country {{alpha_2: Alpha2::{}, alpha_3: Alpha3::{}, numeric: Numeric::N{}, name: \"{}\", official_name: \"{}\"}},",
            line,
            data[1][index],
            data[2][index],
            data[3][index],
            data[4][index],
        ));
    }

    // Update the data
    let file: String = fs::read_to_string(ISO_31661_RUST_FILE).unwrap();
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
    let matcher = RegexMatcher::new(r"(static COUNTRIES:\[Country;[0-9]+\])").expect("Invalid Regex");
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
                    format!("static COUNTRIES:[Country;{}]", data[0].iter().len()).as_str(),
                );
                Ok(true)
            }),
        )
        .unwrap();

    fs::write(ISO_31661_RUST_FILE, new_file);
}

fn update_enum_file(data: &Vec<String>, file_path: &str) {
    println!("Updating {}", file_path);

    let mut codes_enum: String = format!("    {}", data.join(",\n    "));
    if file_path == ISO_31661_NUMERIC_FILE {
        codes_enum = format!("    N{}", data.join(",\n    N"));
    }

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
