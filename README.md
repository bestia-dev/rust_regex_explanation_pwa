[//]: # (auto_md_to_doc_comments segment start A)

# Rust regex explanations in PWA

**Rust regex explanations in PWA**  
***version: 2020.810.640  date: 2020-08-10 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/rust_regex_explanation_pwa)***  

[//]: # (auto_cargo_toml_to_md end)

 ![maintained](https://img.shields.io/badge/maintained-green)
 ![statusready_for_use](https://img.shields.io/badge/ready_for_use-green)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1474-green.svg)](https://github.com/bestia-dev/rust_regex_explanation_pwa/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-156-blue.svg)](https://github.com/bestia-dev/rust_regex_explanation_pwa/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-149-purple.svg)](https://github.com/bestia-dev/rust_regex_explanation_pwa/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/rust_regex_explanation_pwa/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/rust_regex_explanation_pwa/)

[//]: # (auto_lines_of_code end)

 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/rust_regex_explanation_pwa/blob/master/LICENSE)
 [![Rust](https://github.com/bestia-dev/rust_regex_explanation_pwa/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/rust_regex_explanation_pwa/)
 ![rust_regex_explanation_pwa](https://bestia.dev/webpage_hit_counter/get_svg_image/632662465.svg)

Hashtags: #rustlang #tutorial #regex #pwa  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Try it

<https://bestia.dev/rust_regex_explanation_pwa/>

## Regex explanation and testing

Regex is great. But it is much easier to write and understand with a little help of explanations.  
Regex has many flavors with subtle differences. This PWA uses Rust Regex crate.  
It generates Rust code and shows that code results for the 6 important Regex methods.  
Useful as learning tool and as development tool to edit/test/debug your regex first.  

## Minimal example of Wasm/Webassembly with vanilla Html, Css and Javascript

First decision - no frameworks. Just vanilla. Then no javascript.  
Some basic html. Some basic css.  
`contenteditable` - Every day something new. Modern browsers have a wysiwyg editor.  
Just add the attribute contenteditable to a `div` or `code`.  
All the rest is in Rust with web-sys/wasm-bindgen for all the programming needs.  
No other special requirements.  
Ok, I had to use the javascript library `highlightjs` to bring some colors to the code.  
It is more like a coding tutorial for creating PWA in Rust.  

## PWA

I added the manifest, the worker and a bunch of icons.  
Now it :  

- can work offline,  
- can install as an "app"  
- execution is sand-boxed in the browser  
- no way to bring some malware (if the browser in not compromised)  
- stores text and section visibility in LocalStorage of the browser  

## Change colors

Users can change the colors with the use of the Chrome extension `User CSS`:  
<https://chrome.google.com/webstore/detail/user-css/okpjlejfhacmgjkmknjhadmkdbcldfcb>  
Copy/paste and then edit the colors and watch changes alive:  

```css
:root {
    /* color palette */
    /* background color */
    --b_color_body: #24292E;
    --b_color_code: #1B1D23;
    --b_color_grid_header: #181A1F;
    --b_color_container: #333842;
    --b_color_button: dodgerblue;
    /* front color */
    --f_color_body: #dce0e9;
    --f_color_code: #78C379;
    --f_color_link: white;
    --f_color_05: yellow;
    --f_color_06: dark-white;
    --f_color_07: black;
    /* border color*/
    --brd_color_01: #000;
    --brd_color_02: #eaecef;
}
```

## Development

We will need a web file server because security does not allow loading modules from local file.  
Install this basic one:
`cargo install basic-http-server`  
Run the server in a separate terminal so it can stay running all the time:  
Go to the content folder:  
`cd rustprojects/rust_regex_explanation_pwa/web_server_folder/web_content_folder`  
`basic-http-server`  
Open the browser on:  
`http://127.0.0.1:4000`  

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
