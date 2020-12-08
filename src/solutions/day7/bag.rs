use std::collections::HashMap;
use std::str::FromStr;
use std::string::ParseError;
use regex::Regex;


pub struct BagRule {
    pub color: String,
    pub contained_bags: HashMap<String, usize>
}


impl FromStr for BagRule {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut contained_bags: HashMap::<String, usize> = HashMap::new();

        let color_and_contents_re: Regex = Regex::new(r" bags contain ").unwrap();
        let color_and_contents: Vec<&str> = color_and_contents_re.split(s).collect();

        let color = color_and_contents.get(0).unwrap().to_string();
        let contents_re: Regex = Regex::new(r"(?P<count>\d+) (?P<color>[a-z ]+) bags?[,.]").unwrap();

        for capture in contents_re.captures_iter(color_and_contents.get(1).unwrap()) {
            let color = &capture["color"].to_string();
            let count = usize::from_str(&capture["count"]).unwrap();
            contained_bags.insert(color.to_owned(), count);
        }

        Ok(BagRule {
            color: color,
            contained_bags: contained_bags
        })
    }
}

