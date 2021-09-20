//use grep_regex::RegexMatcher;
//use grep_searcher::sinks::UTF8;
//use grep_searcher::{Searcher};
//use grep_matcher::{Captures, Matcher};
use workspace_utils::{
    match_lines,
//    replace_file_content,
//    update_enum_file
};

//use std::fs;
//use std::path::Path;
use std::collections::HashMap;

const URL_SOURCE: &str = "https://download.geonames.org/export/dump/allCountries.zip";
const FILE_SOURCE_DETAILS: &str = "raw_data/allCountries.txt";
//const FILE_RUST_DATA_DETAILS: &str = "src/country/data_details.rs";


fn main() {
    //let _ = download_data_source();
    /*
geonameid         : integer id of record in geonames database
name              : name of geographical point (utf8) varchar(200)
asciiname         : name of geographical point in plain ascii characters, varchar(200)
alternatenames    : alternatenames, comma separated, ascii names automatically transliterated, convenience attribute from alternatename table, varchar(10000)
latitude          : latitude in decimal degrees (wgs84)
longitude         : longitude in decimal degrees (wgs84)
feature class     : see http://www.geonames.org/export/codes.html, char(1)
feature code      : see http://www.geonames.org/export/codes.html, varchar(10)
country code      : ISO-3166 2-letter country code, 2 characters
cc2               : alternate country codes, comma separated, ISO-3166 2-letter country code, 200 characters
admin1 code       : fipscode (subject to change to iso code), see exceptions below, see file admin1Codes.txt for display names of this code; varchar(20)
admin2 code       : code for the second administrative division, a county in the US, see file admin2Codes.txt; varchar(80) 
admin3 code       : code for third level administrative division, varchar(20)
admin4 code       : code for fourth level administrative division, varchar(20)
population        : bigint (8 byte int) 
elevation         : in meters, integer
dem               : digital elevation model, srtm3 or gtopo30, average elevation of 3''x3'' (ca 90mx90m) or 30''x30'' (ca 900mx900m) area in meters, integer. srtm processed by cgiar/ciat.
timezone          : the iana timezone id (see file timeZone.txt) varchar(40)
modification date : date of last modification in yyyy-MM-dd format
    */
    let data: HashMap<&str, Vec<String>> = match_lines(
        FILE_SOURCE_DETAILS,
        r#"(?m)^.*?\t(.*?)\t.*?\t.*?\t(.*?)\t(.*?)\t.*?\tADM3\tCA\t.*?\t.*?\t.*?\t.*?\t.*?\t(.*?)\t.*\t.*\t(.*?)\t"#,
        vec!["name", 
        //"subdivision_alpha2", 
        "lat", "lng", 
        //"country_alpha2", 
        "population", "timezone"]
    );
    //let mut names = data.get("name").unwrap().clone();
    //names.sort();
    println!("{:?}", data);
}

/*
fn download_data_source() -> Result<(), ureq::Error> {
    if Path::new(FILE_SOURCE_DETAILS).exists() {
        return Ok(());
    }

    let content = ureq::get(URL_SOURCE).call()?.into_string()?;
    let _ = fs::write(FILE_SOURCE_DETAILS, content);

    Ok(())
}
*/
