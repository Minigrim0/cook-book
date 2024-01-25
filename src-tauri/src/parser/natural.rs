use regex::Regex;

use crate::database::insertables::NewIngredient;

fn normalize_units(line: &str) -> &str {
    // line.replace(from, to)
    ""
}

pub fn parse_natural_ingredient(line: &str) -> Option<NewIngredient> {
    let basic_re = Regex::new(r"^(?P<amount>\d+\.?\d*) (?:\((?P<alt>.*)\))?\s*(?P<unit>\w+)(?:\s*(?P<name>[.[^,\n]]*))?(?:,(?P<details>.*))?$").unwrap();
    None
}
