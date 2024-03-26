mod characters;
mod media;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test(query: &str) -> () {
    characters::search(query);
}
