use regex::Regex;
use std::fs;

fn remove_comments(input: &str) -> String {
    Regex::new(r#"// .*|(?s)/\*.*?\*/"#).unwrap()
        .replace_all(input, |caps: &regex::Captures| {
            caps.get(0).unwrap().as_str().chars()
                .map(|c| if c == '\n' || c == '\r' { c } else { ' ' })
                .collect::<String>()
        }).to_string()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let output = remove_comments(&input);

    assert_eq!(input.lines().count(), output.lines().count(), "Number of lines changed!");

    println!("{}", output);
}
