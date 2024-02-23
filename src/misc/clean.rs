use regex::Regex;

// Define a tuple to hold language-specific comment syntax
type Language = (&'static str, &'static str, &'static str, &'static str);

// Function to remove comments from code based on language syntax
fn remove_comments(code: &str, lang: &Language) -> String {
    let single_re = Regex::new(&format!(
        r"\s*{}[^\n\r]*[\n\r]*",
        regex::escape(lang.1)
    ))
    .unwrap();
    let multi_re = Regex::new(&format!(
        r"\s*(?s){}.*?{}.*?[\n\r]*",
        regex::escape(lang.2),
        regex::escape(lang.3)
    ))
    .unwrap();

    let code = single_re.replace_all(&code, "");
    let code = multi_re.replace_all(&code, "");

    code.into_owned()
}

fn main() {
    // Define supported languages
    let langs: Vec<Language> = vec![
        ("C", "//", "/*", "*/"),
        ("C++", "//", "/*", "*/"),
        ("Java", "//", "/*", "*/"),
        ("C#", "//", "/*", "*/"),
    ];

    // Example usage
    let code = r#"
        // Single line comment
        /* Multi
           line
           comment */
        int main() {
            // Hello, world!
            return 0;
        }
    "#;

    // Remove comments for each language
    for lang in &langs {
        println!("{} code without comments:\n{}", lang.0, remove_comments(code, lang));
    }
}

