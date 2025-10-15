use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/bundles/shiki.js")]
extern "C" {
    #[wasm_bindgen(js_name = "codeToHtml")]
    pub fn code_to_html(code: &str, lang: &str, theme: &str) -> String;
}
