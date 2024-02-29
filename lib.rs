use regex::Regex;
use lazy_static::lazy_static;

lazy_static! { static ref RE: Regex = Regex::new(r#"// .*|(?s)/\*.*?\*/"#).unwrap(); }

pub fn keep_newlines_spaces(input: &str) -> String {
    RE.replace_all(input, |caps: &regex::Captures| caps[0].chars()
            .map(|c| if c == '\n' || c == '\r' { c } else { ' ' })
            .collect::<String>()).to_string()
}

pub fn keep_newlines(input: &str) -> String {
    RE.replace_all(input, |caps: &regex::Captures|
            caps[0].chars()
                .filter(|&c| c == '\n' || c == '\r')
                .collect::<String>()).to_string()
}

pub fn keep_nothing(input: &str) -> String {
    RE.replace_all(input, "").to_string()
}
