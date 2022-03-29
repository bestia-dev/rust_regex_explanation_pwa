use crate::html_encoded_push;
use crate::web_sys_mod::HtmlEncoded;

/// the variables must be html_encoded
pub fn code_gen_html(regex_text: &str, substitution: &str, test_string: &str) -> HtmlEncoded {
    let mut html = HtmlEncoded::new();
    init_code_template(&mut html);
    html.replace_with_html_encode("Luciano(Best)ia", regex_text);
    html.replace_with_html_encode("OnlyThe$1", substitution);
    html.replace_with_html_encode(r###"origin  git@github.com:LucianoBestia/rust_regex_explanation_pwa.git (fetch)\norigin  https://github.com/LucianoBestia/rust_regex_explanation_pwa (fetch)"###,test_string);
    // return
    html
}

/// variables must be not html_encoded
pub fn code_gen_string(regex_text: &str, substitution: &str, test_string: &str) -> String {
    let mut html = HtmlEncoded::new();
    init_code_template(&mut html);
    let html = html.get_html();
    let html = html
    .replace("Luciano(Best)ia",regex_text)
    .replace("OnlyThe$1", substitution)
    .replace(r###"origin  git@github.com:LucianoBestia/rust_regex_explanation_pwa.git (fetch)\norigin  https://github.com/LucianoBestia/rust_regex_explanation_pwa (fetch)"###,test_string);
    // return
    html
}

/// the code template
fn init_code_template(html: &mut HtmlEncoded) {
    // This template must be a string literal because
    // it will be used in a format! macro.
    // This is why I need to replace all { with {{ and } with }} manually in the literal.
    html_encoded_push!(
        html,
        r#####"// Rust Regex code-gen  
// https://github.com/LucianoBestia/rust_regex_explanation_pwa  
// Run this code online in the playground:  
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=10717a3fe934b9583fb675e327833edc  

// There are 6 important Regex methods for different use-cases:
// is_match(), find(), find_iter(), capture(), capture_iter(), replace_all()

// In Cargo.toml include the dependency to the regex crate:  
// regex = "1.3.6"  
use regex::Regex;

// To avoid multiple initialization of the regex and achieve better performance,
// especially when the regex is used in a loop include also the lazy_static crate:
// lazy_static="1.4.0"
use lazy_static::lazy_static;

// Use "raw strings" syntax to avoid unnecessary escaping.
// It will leave the regex expression unmodified. No more problems with \ or " characters.
// Raw string syntax is like: `r#"one two three"#` or `r##"one two three"##`  
// If the expression is not correct, we want to know it immediately.
// Let the constructor panic on error with `unwrap()`.
lazy_static! {{
    static ref RGX_01: Regex = Regex::new(r###"Luciano(Best)ia"###).unwrap();
}}

fn main() {{
    println!("--- rust_regex_explanation_pwa start ---");

    // prepared example
    let test_string = r###"origin  git@github.com:LucianoBestia/rust_regex_explanation_pwa.git (fetch)\norigin  https://github.com/LucianoBestia/rust_regex_explanation_pwa (fetch)"###;
    // substitution for replace_all()
    // the $1, $2,.. are placeholders for the found capture group
    let substitution = r###"OnlyThe$1"###;

    // 1.uncomment for is_match = false
    //let rgx: &Regex = &Regex::new(r#"xxx"#).unwrap();
    // 2. uncomment for is_match = true
    let rgx = &RGX_01;

    is_match(&rgx, &test_string);
    find(&rgx, &test_string);
    find_iter(&rgx, &test_string);
    captures(&rgx, &test_string);
    captures_iter(&rgx, &test_string);
    replace_all(&rgx, &test_string, substitution);

    println!("--- rust_regex_explanation_pwa end ---");
}}

/// example how to use the is_match() method
fn is_match(rgx: &Regex, test_string: &str) {{
    if rgx.is_match(test_string) {{
        println!("True - is match.");
    }} else {{
        println!("False - no match.");
    }}
}}

/// example how to find the first occurrence
fn find(rgx: &Regex, test_string: &str) {{
    // method find() returns Option:None if not found.
    // There are more than one way in Rust to check for `possibility of absence`.
    // The first way is the methods unwrap() or expect(),
    // but they are good only for tests and examples. Never use them in production code.

    // using pattern matching (match Control Flow Operator) for `case analysis `.
    match rgx.find(test_string) {{
        Some(m) => println!("1. find: {{}} {{}} {{}}", m.start(), m.end(), m.as_str()),
        None => println!("1. find: None"),
    }}
    // using `if let`syntax
    if let Some(m) = rgx.find(test_string) {{
        println!("2. find: {{}} {{}} {{}}", m.start(), m.end(), m.as_str());
    }} else {{
        println!("2. find: None");
    }}

    // using map_or_else() combinator
    rgx.find(test_string).map_or_else(
        || println!("3. find: None"),
        |m| println!("3. find: {{}} {{}} {{}}", m.start(), m.end(), m.as_str()),
    );
}}

/// example how to use find_iter() method - iterator
fn find_iter(rgx: &Regex, test_string: &str) {{
    println!("find_iter start");
    for m in rgx.find_iter(test_string) {{
        println!("find_iter: {{}} {{}} {{}}", m.start(), m.end(), m.as_str())
    }}
    println!("find_iter end");
}}

/// example how to capture only the first occurrence of rgx capture groups
/// using the captures() method for regex capture groups
fn captures(rgx: &Regex, test_string: &str) {{
    println!("captures start");
    // same 3 possible syntax to react to the `possibility of absence` Option:None
    // as in the function find()
    match rgx.captures(test_string) {{
        Some(m) => {{
            // the whole match
            println!("    match:    {{}}..{{}} {{}}\n",
                &m.get(0).unwrap().start().to_string(),
                &m.get(0).unwrap().end().to_string(),
                m.get(0).unwrap().as_str()
            );
            // every group captured inside the match
            for i in 1..m.len() {{
              println!("    {{}}. group: {{}}..{{}} {{}}\n",
                    &i.to_string(),
                    &m.get(i).unwrap().start().to_string(),
                    &m.get(i).unwrap().end().to_string(),
                    m.get(i).unwrap().as_str()
                );
            }}
        }}
        None => println!("1. captures: None"),
    }}
    println!("captures end");
}}

/// example how to use captures_iter() method - iterator
fn captures_iter(rgx: &Regex, test_string: &str) {{
    println!("captures_iter start");
    for m in rgx.captures_iter(test_string) {{
        // the whole match
        println!("    match:    {{}}..{{}} {{}}\n",
            &m.get(0).unwrap().start().to_string(),
            &m.get(0).unwrap().end().to_string(),
            m.get(0).unwrap().as_str()
        );
        // every group captured inside the match
        for i in 1..m.len() {{
            println!("    {{}}. group: {{}}..{{}} {{}}\n",
                &i.to_string(),
                &m.get(i).unwrap().start().to_string(),
                &m.get(i).unwrap().end().to_string(),
                m.get(i).unwrap().as_str()
            );
        }}
    }}
    println!("captures_iter end");
}}

/// example of how to use replace_all() method
/// the $1, $2,.. are placeholders for the found capture group
fn replace_all(rgx: &Regex, test_string: &str, replace_string: &str) {{
    println!("replace_all start");
    let new_string = rgx.replace_all(test_string, replace_string).to_string();
    println!("replaced:\n{{}}", new_string);
    println!("replace_all end");
}}
    "#####
    );
}
