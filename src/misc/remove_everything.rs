use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let output = Regex::new(r#"// .*|(?s)/\*.*?\*/"#).unwrap().replace_all(&input, "");

    println!("{}", output);
}
