use std::str::FromStr;
use std::string::ParseError;

pub struct PasswordOccurenceRange {
    pub min: usize,
    pub max: usize,
}

impl FromStr for PasswordOccurenceRange {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let range: Vec<&str> = s.split('-')
                                .collect();

        let min = range[0].parse::<usize>().unwrap();
        let max = range[1].parse::<usize>().unwrap();

        Ok(PasswordOccurenceRange {
            min: min,
            max: max,
        })
    }
}


pub struct PasswordPolicy {
    pub letter: char,
    pub range: PasswordOccurenceRange
}

impl FromStr for PasswordPolicy {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let policy_list: Vec<&str> = s.split(' ')
                                      .collect();

        let range = PasswordOccurenceRange::from_str(policy_list[0]).unwrap();
        let letter = policy_list[1].to_string().pop().unwrap();
        Ok(PasswordPolicy {
            letter: letter,
            range: range
        })
    }
}


pub struct PasswordWithPolicy {
    pub password: String,
    pub policy: PasswordPolicy
}

impl FromStr for PasswordWithPolicy {
     type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let password_with_policy: Vec<&str> = s.split(':')
                                               .collect();

        let policy = PasswordPolicy::from_str(password_with_policy[0]).unwrap();
        let chars_to_trim: &[char] = &[' '];
        let password = password_with_policy[1].trim_matches(chars_to_trim).to_string();
        Ok(PasswordWithPolicy {
            password: password,
            policy: policy
        })
    }
}
