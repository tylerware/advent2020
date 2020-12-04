use std::str::FromStr;
use std::string::ParseError;
use regex::Regex;

pub struct Credential {
    pub birth_year: String,
    pub issue_year: String,
    pub expiration_year: String,
    pub height: String,
    pub eye_color: String,
    pub hair_color: String,
    pub passport_id: String,
    pub country_id: String,
}

impl Credential {
    // Checks that all fields are present (with the exception of country_id, which we ignore)
    pub fn has_north_pole_credential_fields(&self) -> bool {
        self.birth_year != ""
            && self.issue_year != ""
            && self.expiration_year != ""
            && self.height != ""
            && self.eye_color != ""
            && self.hair_color != ""
            && self.passport_id != ""
    }

    pub fn is_valid_north_pole_credential(&self) -> bool {
        self.has_north_pole_credential_fields()
            && self.valid_birth_year()
            && self.valid_issue_year()
            && self.valid_expiration_year()
            && self.valid_height()
            && self.valid_hair_color()
            && self.valid_eye_color()
            && self.valid_passport_id()
    }

    fn valid_birth_year(&self) -> bool {
        let birth_year = usize::from_str(&self.birth_year).unwrap();
        birth_year >= 1920 && birth_year <= 2002
    }

    fn valid_issue_year(&self) -> bool {
        let issue_year = usize::from_str(&self.issue_year).unwrap();
        issue_year >= 2010 && issue_year <= 2020
    }

    fn valid_expiration_year(&self) -> bool {
        let expiration_year = usize::from_str(&self.expiration_year).unwrap();
        expiration_year >= 2020 && expiration_year <= 2030
    }

    fn valid_height(&self) -> bool {
        let regex: Regex = Regex::new(r"^(?P<value>[\d]+)(?P<unit>(cm|in))$").unwrap();
        let mut valid = false;
        for capture in regex.captures_iter(&self.height) {
            let value = usize::from_str(&capture["value"]).unwrap();
            valid = match &capture["unit"] {
                "cm" => (value >= 150 && value <= 193),
                "in" => (value >= 59 && value <= 76),
                _ => false
            }
        }
        valid
    }

    fn valid_hair_color (&self) -> bool {
        let regex: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        regex.is_match(&self.hair_color)
    }

    fn valid_eye_color (&self) -> bool {
        let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        colors.iter().any(|color| &self.eye_color == color)
    }

    fn valid_passport_id(&self) -> bool {
        let regex: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        regex.is_match(&self.passport_id)
    }
}

impl FromStr for Credential {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut credential = Credential {
            birth_year: "".to_string(),
            issue_year: "".to_string(),
            expiration_year: "".to_string(),
            height: "".to_string(),
            eye_color: "".to_string(),
            hair_color: "".to_string(),
            passport_id: "".to_string(),
            country_id: "".to_string(),
        };

        let regex: Regex = Regex::new(r"(?P<field>[^\s]+):(?P<value>[^\s]+)").unwrap();
        for capture in regex.captures_iter(s) {
            let value = &capture["value"];
            let field = &capture["field"];
            match field {
                "byr" => credential.birth_year = value.to_string(),
                "iyr" => credential.issue_year = value.to_string(),
                "eyr" => credential.expiration_year = value.to_string(),
                "hgt" => credential.height = value.to_string(),
                "ecl" => credential.eye_color = value.to_string(),
                "hcl" => credential.hair_color = value.to_string(),
                "pid" => credential.passport_id = value.to_string(),
                "cid" => credential.country_id = value.to_string(),
                _ => println!("Unrecognized field: {}", field)
            }
        }

        Ok(credential)
    }
}
