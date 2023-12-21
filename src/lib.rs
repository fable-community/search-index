use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

// macro_rules! console_log {
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

#[wasm_bindgen]
pub fn levenshtein(a: String, b: String) -> u32 {
    console_error_panic_hook::set_once();
    triple_accel::levenshtein(&a.into_bytes(), &b.into_bytes())
}
