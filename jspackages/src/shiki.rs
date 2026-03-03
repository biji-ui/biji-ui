#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/bundles/shiki.js")]
extern "C" {
    #[wasm_bindgen(js_name = "codeToHtml")]
    pub fn code_to_html(code: &str, lang: &str, theme: &str) -> String;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn code_to_html(_code: &str, _lang: &str, _theme: &str) -> String {
    String::new()
}
