use regex::Regex;

#[cfg(feature = "database")]
pub fn iso8601_to_seconds(iso_string: String) -> i32 {
    let re = Regex::new(r"(?:P?)(?:([0-9]+)Y)?(?:([0-9]+)M)?(?:([0-9]+)D)?(?:T?)(?:([0-9]+)H)?(?:([0-9]+)M)?(?:([0-9]+)S)?").unwrap();
    if let Some(matches) = re.captures(&iso_string) {
        matches.get(3).map_or("", |m| m.as_str()).parse::<i32>().ok().unwrap_or(0) * 24 * 3600  // D
        + matches.get(4).map_or("", |m| m.as_str()).parse::<i32>().ok().unwrap_or(0) * 3600     // H
        + matches.get(5).map_or("", |m| m.as_str()).parse::<i32>().ok().unwrap_or(0) * 60       // M
        + matches.get(6).map_or("", |m| m.as_str()).parse::<i32>().ok().unwrap_or(0)            // S
    } else {
        -1
    }
}