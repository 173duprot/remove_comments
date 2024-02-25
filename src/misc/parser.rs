use std::fs;
use syn::{parse_file, visit_mut::VisitMut, File, Attribute, LitStr};

struct CommentRemover;

impl VisitMut for CommentRemover {
    fn visit_attribute_mut(&mut self, i: &mut Attribute) {
        // This is a doc comment, so we replace it with an equivalent number of newlines.
        let line_count = i.tokens.to_string().chars().filter(|&c| c == '\n').count();
        let new_content = "\n".repeat(line_count);
        i.tokens = syn::parse_quote!(#[doc = #new_content]);
    }

    fn visit_lit_str_mut(&mut self, i: &mut LitStr) {
        // We don't want to remove comments inside strings, so we do nothing here.
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut ast = parse_file(&input).unwrap();

    CommentRemover.visit_file_mut(&mut ast);

    println!("{}", quote::quote! { #ast });
}
