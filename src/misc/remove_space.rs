use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let output = Regex::new(r#"// .*|(?s)/\*.*?\*/"#).unwrap()
        .replace_all(&input, |caps: &regex::Captures| {
            caps[0].chars().filter(|&c| c == '\n' || c == '\r').collect::<String>()
        }).to_string();

    assert_eq!(input.lines().count(), output.lines().count());
    println!("{}", output);
}