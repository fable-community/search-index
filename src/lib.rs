use wasm_bindgen::prelude::*;

use std::collections::BTreeMap;

mod characters;
mod media;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

fn normalize_text(text: &str) -> String {
    unidecode::unidecode(&text.to_lowercase())
}

pub(crate) fn tokenizer(text: String) -> Vec<String> {
    let words = text.split_whitespace().map(normalize_text);

    words.collect::<Vec<String>>()
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub(crate) struct Index<T> {
    pub(crate) refs: BTreeMap<String, Vec<usize>>,
    pub(crate) data: Vec<T>,
}

pub(crate) trait Insert<T> {
    fn insert(&mut self, item: T);
}

impl<T> Index<T> {
    pub(crate) fn default() -> Self {
        Self {
            refs: BTreeMap::new(),
            data: Vec::new(),
        }
    }
}
