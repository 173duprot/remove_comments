extern crate regex;

use regex::Regex;

fn main() {
    let input = r#"
    This is a block of text. As John Smith says /*to be or 
    not to be*/ but what is the question? // This is a comment
    "#;

    let output = Regex::new(r#"(?s)/\*.*?\*/|//.*$"#).unwrap()
        .replace_all(&input, |caps: &regex::Captures| {
            caps.get(0).unwrap().as_str()
                .chars()
                .map(|c| if c == '\n' || c == '\r' { c } else { ' ' })
                .collect::<String>()
        });

    assert_eq!(input.lines().count(), output.lines().count(), "Number of lines changed!");

    println!("{}", output);
}

