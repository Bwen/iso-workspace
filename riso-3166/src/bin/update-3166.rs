use grep_regex::RegexMatcher;
use grep_searcher::sinks::UTF8;
use grep_searcher::{Searcher, SearcherBuilder};
use grep_matcher::{Captures, Matcher};
// use ureq;
use std::fs;
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
    update_31661();
    update_31662();
}

fn update_31661() {
    let data: HashMap<&str, Vec<String>> = match_lines(
        ISO_31661_DATA_FILE,
        "(?m)^\"([A-Z]{2})\",\"([A-Z]{3})\",\"([0-9]{1,3})\",\".*\",\"officially-assigned\",\"(.*)\",\".*\",\"(.*)\"",
        vec!["alpha2", "alpha3", "numeric", "name", "name_full"]
    );

    update_enum_file(&data.get("alpha2").unwrap().clone(), ISO_31661_ALPHA2_FILE);
    update_enum_file(&data.get("alpha3").unwrap().clone(), ISO_31661_ALPHA3_FILE);
    update_enum_file(&data.get("numeric").unwrap().clone(), ISO_31661_NUMERIC_FILE);
    update_31661_data_file(data);
}

fn update_31662() {
    let mut data = match_lines(
        ISO_31662_SUBDIVISIONS_DATA_FILE,
        "(?m)^\"([A-Z]{2})\",\".*?\",\".*?\",\"([0-9]{3})\",\"([A-Z]{2}-.*)\",\".*\",\"(.*)\"",
        vec!["alpha2", "category_id", "code", "parent"]
    );

    let sanitized_codes: Vec<String> = data.get("code").unwrap().iter().map(|c| {
        let code_parts: Vec<&str> = c.split('-').collect();
        code_parts.join("_")
    }).collect();
    data.entry("sanitized_code").or_insert_with(|| sanitized_codes.clone());
    update_enum_file(&sanitized_codes, ISO_31662_CODE_FILE);
    
    let categories = match_lines(
        ISO_31662_CATEGORIES_DATA_FILE,
        "(?m)^\".*\",\".*\",\".*\",\"([0-9]+)\",\"en\",\".*\",\"(.*)\",\".*\"",
        vec!["id", "text"]
    );

    let names = fetch_31662_names();
    update_31662_data_file(data, categories, names);
}

fn update_31662_data_file(data: HashMap<&str, Vec<String>>, categories: HashMap<&str, Vec<String>>, names: HashMap<String, String>) {
    println!("Updating {}", ISO_31662_SUBDIVISION_RUST_FILE);

    let mut data_strings: Vec<String> = vec![];
    for (index, alpha2) in data.get("alpha2").unwrap().iter().enumerate() {
    //for (index, line) in data[0].iter().enumerate() {
        let category_id = &data.get("category_id").unwrap()[index];
        let mut parent_code = String::from("None");
        if !&data.get("parent").unwrap()[index].is_empty() {
            let parent_code_parts: Vec<&str> = data.get("parent").unwrap()[index].split('-').collect();
            parent_code = parent_code_parts.join("_");
        }
        
        let cat_id_index = &categories.get("id").unwrap().iter().position(|id| id == category_id).unwrap();
        let category = &categories.get("text").unwrap()[*cat_id_index];
        let sanitized_code = &data.get("sanitized_code").unwrap()[index];
        data_strings.push(format!(
            "    Subdivision {{country: Alpha2::{}, code: Code::{}, parent: Code::{}, name: \"{}\", category: \"{}\"}},",
            alpha2,
            sanitized_code,
            parent_code,
            names.get(sanitized_code).unwrap(),
            category,
        ));
    }

    replace_file_content(
        ISO_31662_SUBDIVISION_RUST_FILE, 
        r"(?ms).*?// DATA START\n(.*)\s+// DATA END.*", 
        format!("{}\n   ", data_strings.join("\n")).as_str()
    );

    replace_file_content(
        ISO_31662_SUBDIVISION_RUST_FILE, 
        r"(static SUBDIVISIONS:\[Subdivision;[0-9]+\])", 
        format!("static SUBDIVISIONS:[Subdivision;{}]", data_strings.len()).as_str()
    );
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
                let _ = matcher.captures(line.as_bytes(), &mut captures);
                let text = &line[captures.get(2).unwrap()];
                let code = line[captures.get(1).unwrap()].to_string();
                let code_parts: Vec<&str> = code.split('-').collect();

                // The file `subdivision-names.csv` provided by `iso.org` is confusing,
                // it does not always provide english text for subdivisions, which is insane...
                // So we take the first entry only, it is not guarantee to be in english. Most are?
                let sanitized_text = strip_comment(text, &['(']);
                matches.entry(code_parts.join("_")).or_insert_with(|| sanitized_text.trim().to_string());

                Ok(true)
            }),
        )
        .unwrap();

    matches
}

fn update_31661_data_file(data: HashMap<&str, Vec<String>>) {
    println!("Updating {}", ISO_31661_RUST_FILE);

    let mut data_strings: Vec<String> = vec![];
    for (index, alpha2) in data.get("alpha2").unwrap().iter().enumerate() {
        data_strings.push(format!(
            "    Country {{alpha2: Alpha2::{}, alpha3: Alpha3::{}, numeric: Numeric::N{}, name: \"{}\", official_name: \"{}\"}},",
            alpha2,
            data.get("alpha3").unwrap()[index],
            data.get("numeric").unwrap()[index],
            data.get("name").unwrap()[index],
            data.get("name_full").unwrap()[index],
        ));
    }

    replace_file_content(
        ISO_31661_RUST_FILE, 
        r"(?ms).*?// DATA START\n(.*)\s+// DATA END.*", 
        format!("{}\n   ", data_strings.join("\n")).as_str()
    );

    replace_file_content(
        ISO_31661_RUST_FILE, 
        r"(static COUNTRIES:\[Country;[0-9]+\])", 
        format!("static COUNTRIES:[Country;{}]", data.get("alpha2").unwrap().iter().len()).as_str()
    );
}

fn update_enum_file(data: &[String], file_path: &str) {
    println!("Updating {}", file_path);

    let mut enums: Vec<&str> = data.iter()
        .filter(|v| v.as_str() != "None")
        .map(|v| v.trim())
        .collect();

    enums.sort_unstable();
    enums.dedup();

    let mut codes_enum: String = format!("    {}", enums.join(",\n    "));
    if file_path == ISO_31661_NUMERIC_FILE {
        codes_enum = format!("    N{}", enums.join(",\n    N"));
    }

    replace_file_content(
        file_path, 
        r"(?ms).*?// ENUM START\n(.*)\s+// ENUM END.*", 
        format!("{},\n   ", &codes_enum).as_str()
    );
}

fn match_lines<'a>(file_path: &'a str, regex: &'a str, result_map: Vec<&'a str>) -> HashMap<&'a str, Vec<String>> {
    let matcher = RegexMatcher::new_line_matcher(regex).unwrap();
    let mut captures = matcher.new_captures().unwrap();
    let mut matches: HashMap<&str, Vec<String>> = HashMap::new();
    Searcher::new()
        .search_path(
            &matcher,
            file_path,
            UTF8(|_, line| {
                let _ = matcher.captures(line.as_bytes(), &mut captures);
                for (i, index) in result_map.iter().enumerate() {
                    let entry = matches.entry(index).or_insert_with(Vec::new);
                    entry.push(line[captures.get(i + 1).unwrap()].to_string());
                }

                Ok(true)
            }),
        )
        .unwrap();

    matches
}

fn replace_file_content(file_path: &str, regex: &str, replacement: &str) {
    let file: String = fs::read_to_string(file_path).unwrap();
    let matcher = RegexMatcher::new(regex).expect("Invalid Regex");
    let mut captures = matcher.new_captures().unwrap();

    let mut new_file: String = String::from("");
    SearcherBuilder::new()
        .multi_line(true)
        .build()
        .search_slice(
            &matcher,
            file.as_bytes(),
            UTF8(|_, result| {
                let _ = matcher.captures(result.as_bytes(), &mut captures);
                new_file = file.replace(
                    &result[captures.get(1).unwrap()].to_string(),
                    replacement,
                );

                Ok(true)
            }),
        )
        .unwrap();

    let _ = fs::write(file_path, new_file);
}
