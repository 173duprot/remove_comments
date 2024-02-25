use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    let comment_regex = r#"//.*(?:\n|\r\n?)|(?s)/\*.*?\*/"#;
    let output = Regex::new(comment_regex).unwrap()
        .replace_all(&input, |caps: &regex::Captures| {
            if let Some(comment) = caps.get(0) {
                let matched_text = comment.as_str();
                matched_text
                    .chars()
                    .map(|c| if c == '\n' || c == '\r' { c } else { ' ' })
                    .collect::<String>()
            } else {
                String::new()
            }
        });

    assert_eq!(input.lines().count(), output.lines().count(), "Number of lines changed!");

    println!("{}", output);
}
