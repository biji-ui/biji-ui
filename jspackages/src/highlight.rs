use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/output/highlight.js")]
extern "C" {
    pub fn highlight(code: &str, language: &str) -> String;

    #[wasm_bindgen(js_name = "highlightElement")]
    pub fn highlight_element(element: &web_sys::HtmlElement);
}
