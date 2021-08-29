use std::fs;
use grep_regex::RegexMatcher;
use grep_searcher::sinks::UTF8;
use grep_searcher::{Searcher, SearcherBuilder};
use grep_matcher::{Captures, Matcher};
use std::collections::HashMap;

pub fn update_enum_file(data: &[String], file_path: &str) {
    println!("Updating {}", file_path);

    let mut enums: Vec<String> = data.iter()
        .filter(|v| v.as_str() != "None" && !v.is_empty() && v.len() != 1)
        .map(|v| v.to_ascii_uppercase())
        .collect();

    enums.sort_unstable();
    enums.dedup();

    let codes_enum: String = format!("    {}", enums.join(",\n    "));
    replace_file_content(
        file_path, 
        r"(?ms).*?// ENUM START\n(.*)\s+// ENUM END.*", 
        format!("{},\n   ", &codes_enum).as_str()
    );
}

pub fn match_lines<'a>(file_path: &'a str, regex: &'a str, result_map: Vec<&'a str>) -> HashMap<&'a str, Vec<String>> {
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

pub fn replace_file_content(file_path: &str, regex: &str, replacement: &str) {
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
