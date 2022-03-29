// region: lmake_md_to_doc_comments include README.md A //!
//! # rust_regex_explanation_pwa
//!
//! ***version: 2020.810.640  date: 2020-08-10 authors: Luciano Bestia***  
//! **Rust regex explanations in PWA**
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1474-green.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-156-blue.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-149-purple.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//!
//! [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/rust_regex_explanation_pwa/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//!
//! ## Try it
//!
//! <https://bestia.dev/rust_regex_explanation_pwa/>
//!
//! ## Regex explanation and testing
//!
//! Regex is great. But it is much easier to write and understand with a little help of explanations.  
//! Regex has many flavors with subtle differences. This PWA uses Rust Regex crate.  
//! It generates Rust code and shows that code results for the 6 important Regex methods.  
//! Useful as learning tool and as development tool to edit/test/debug your regex first.  
//!
//! ## Minimal example of Wasm/Webassembly with vanilla Html, Css and Javascript
//!
//! First decision - no frameworks. Just vanilla. Then no javascript.  
//! Some basic html. Some basic css.  
//! `contenteditable` - Every day something new. Modern browsers have a wysiwyg editor.  
//! Just add the attribute contenteditable to a `div` or `code`.  
//! All the rest is in Rust with web-sys/wasm-bindgen for all the programming needs.  
//! No other special requirements.  
//! Ok, I had to use the javascript library `highlightjs` to bring some colors to the code.  
//! It is more like a coding tutorial for creating PWA in Rust.  
//!
//! ## PWA
//!
//! I added the manifest, the worker and a bunch of icons.  
//! Now it :  
//!
//! - can work offline,  
//! - can install as an "app"  
//! - execution is sand-boxed in the browser  
//! - no way to bring some malware (if the browser in not compromised)  
//! - stores text and section visibility in LocalStorage of the browser  
//!
//! ## Change colors
//!
//! Users can change the colors with the use of the Chrome extension `User CSS`:  
//! <https://chrome.google.com/webstore/detail/user-css/okpjlejfhacmgjkmknjhadmkdbcldfcb>  
//! Copy/paste and then edit the colors and watch changes alive:  
//!
//! ```css
//! :root {
//!     /* color palette */
//!     /* background color */
//!     --b_color_body: #24292E;
//!     --b_color_code: #1B1D23;
//!     --b_color_grid_header: #181A1F;
//!     --b_color_container: #333842;
//!     --b_color_button: dodgerblue;
//!     /* front color */
//!     --f_color_body: #dce0e9;
//!     --f_color_code: #78C379;
//!     --f_color_link: white;
//!     --f_color_05: yellow;
//!     --f_color_06: dark-white;
//!     --f_color_07: black;
//!     /* border color*/
//!     --brd_color_01: #000;
//!     --brd_color_02: #eaecef;
//! }
//! ```
//!
// endregion: lmake_md_to_doc_comments include README.md A //!

use std::env;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys_mod::*;

mod code_gen_mod;
mod examples_mod;
mod regex_explanation_mod;
mod regex_method_mod;
mod test_string_mod;
mod web_sys_mod;

use crate::examples_mod::*;

/// To start the Wasm application, wasm_bindgen runs this functions
#[wasm_bindgen(start)]
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    debug_write("--- rust_regex_explanation_pwa start ---");

    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    let html_encoded = HtmlEncoded::from_str(env!("CARGO_PKG_VERSION"));
    set_element_inner_html_by_id("pkg_version", &html_encoded);

    // Initialize input fields
    init_texts();
    // hide/show sections
    if load_string_from_local_storage("examples_section.display", "none") == "none" {
        display_none_2("examples_section", "examples_label");
    }
    if load_string_from_local_storage("explanation_section.display", "none") == "none" {
        display_none_2("explanation_section", "explanation_label");
    }
    if load_string_from_local_storage("result_label.display", "none") == "none" {
        display_none_2("result_label", "result_section");
    }
    if load_string_from_local_storage("code_gen_section.display", "none") == "none" {
        display_none_2("code_gen_section", "code_gen_label");
    }
    if load_string_from_local_storage("regex_help_label.display", "none") == "none" {
        display_none_2("regex_help_label", "regex_help_section");
    }

    set_all_event_listeners();

    Ok(())
}

// set all event handlers for DOM elements
fn set_all_event_listeners() {
    // parameters: element_is, function , arg_1, arg_2, ...  these are static arguments
    on_click!(
        "menu_examples",
        display_block_2_and_scroll,
        "examples_section",
        "examples_label",
        "examples_label"
    );
    on_click!(
        "menu_explanation",
        display_block_2_and_scroll,
        "explanation_section",
        "explanation_label",
        "explanation_label"
    );
    on_click!(
        "menu_result",
        display_block_2_and_scroll,
        "result_section",
        "result_label",
        "result_label"
    );
    on_click!(
        "menu_code_gen",
        display_block_2_and_scroll,
        "code_gen_section",
        "code_gen_label",
        "code_gen_label"
    );
    on_click!(
        "menu_regex_help",
        display_block_2_and_scroll,
        "regex_help_section",
        "regex_help_label",
        "regex_help_label"
    );

    on_keyup!("regex_text", run_regex);
    on_keyup!("substitution", run_regex);
    on_click!("test_string_less", change_height, "test_string", "150px");
    on_click!("test_string_more", change_height, "test_string", "auto");
    on_click!("explanation_close", display_none_2, "explanation_section", "explanation_label");
    on_click!("explanation_less", change_height, "explanation", "150px");
    on_click!("explanation_more", change_height, "explanation", "auto");
    on_click!("result_close", display_none_2, "result_label", "result_section");
    on_click!("result_less", change_height, "result", "150px");
    on_click!("result_more", change_height, "result", "auto");
    on_click!("code_gen_close", display_none_2, "code_gen_section", "code_gen_label");
    on_click!("code_gen_less", change_height, "code_gen", "150px");
    on_click!("code_gen_more", change_height, "code_gen", "auto");
    on_click!("examples_close", display_none_2, "examples_section", "examples_label");
    on_click!("examples_less", change_height, "examples", "150px");
    on_click!("examples_more", change_height, "examples", "auto");
    on_click!("regex_help_close", display_none_2, "regex_help_label", "regex_help_section");
    on_click!("regex_help_less", change_height, "regex_help", "150px");
    on_click!("regex_help_more", change_height, "regex_help", "auto");

    on_click!("example_email", example_email);
    on_click!("example_model_1", example_model_1);
    on_click!("example_model_2", example_model_2);
    on_click!("example_model_3", example_model_3);
    on_click!("example_xml_1", example_xml_1);
    on_click!("example_xml_2", example_xml_2);
    on_click!("example_xml_3", example_xml_3);

    on_click!("code_gen_copy", code_gen_copy);
    on_click!("code_gen_run_in_playground", code_gen_run_in_playground);
}

/// on keyup code
fn run_regex() {
    let regex_text = get_text_area_element_value_string_by_id("regex_text");
    let substitution = get_text_area_element_value_string_by_id("substitution");
    let test_string = get_element_inner_text_by_id("test_string");

    //save the texts to local storage
    save_to_local_storage("regex_text", &regex_text);
    save_to_local_storage("substitution", &substitution);
    save_to_local_storage("test_string", &test_string);

    let explanation = regex_explanation_mod::create_explanation_html(regex_text.clone());
    set_element_inner_html_by_id("explanation", &explanation);

    let result = regex_method_mod::run_regex_methods_html(&regex_text, &substitution, &test_string);
    set_element_inner_html_by_id("result", &result);

    let code_gen = code_gen_mod::code_gen_html(&regex_text, &substitution, &test_string);
    set_element_inner_html_by_id("code_gen", &code_gen);

    let test_string = test_string_mod::test_string_html(&regex_text, &test_string);
    set_element_inner_html_by_id("test_string", &test_string);

    // Applies highlighting to the blocks on a page.
    unwrap!(js_sys::eval("hljs.highlightBlock(document.getElementById('code_gen'))"));
}

/// make visible 2 elements and jump to the third element
fn display_block_2_and_scroll(element_1_id: &str, element_2_id: &str, element_3_id: &str) {
    display_block(element_1_id);
    display_block(element_2_id);
    scroll(element_3_id);
}

/// make 2 invisible
fn display_none_2(element_1_id: &str, element_2_id: &str) {
    display_none(element_1_id);
    display_none(element_2_id);
}

// copy to clipboard
fn code_gen_copy() {
    let regex_text = get_text_area_element_value_string_by_id("regex_text");
    let substitution = get_text_area_element_value_string_by_id("substitution");
    let test_string = get_element_inner_text_by_id("test_string");
    let code_gen = code_gen_mod::code_gen_string(&regex_text, &substitution, &test_string);
    copy_to_clipboard(&code_gen);
}

// open playground URL in new window
fn code_gen_run_in_playground() {
    let uri = "https://play.integer32.com/?version=stable&mode=debug&edition=2018&code=ctrl+a, ctrl+v = Paste here the code from the PWA Rust Regex and then click Run.";
    open_url_in_new_tab(&uri);
}
