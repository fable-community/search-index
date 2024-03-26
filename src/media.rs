use serde_json::from_str;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, serde::Serialize, serde::Deserialize, Clone,
)]
struct Media {
    id: String,
    title: Vec<String>,
    popularity: u32,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Default)]
struct Index {
    refs: BTreeMap<String, Vec<usize>>,
    data: Vec<Media>,
}

impl Index {
    fn insert(&mut self, item: Media) -> () {
        let i = self.data.len();

        let mut combined = Vec::new();

        combined.extend(&item.title);

        for s in &combined {
            let terms = tokenizer((*s).clone());

            for term in terms {
                self.refs.entry(term).or_insert_with(Vec::new).push(i);
            }
        }

        self.data.push(item);
    }
}

fn normalize_text(text: &str) -> String {
    unidecode::unidecode(&text.to_lowercase())
}

fn tokenizer(text: String) -> Vec<String> {
    let words = text.split_whitespace().map(normalize_text);

    words.collect::<Vec<String>>()
}

pub fn create_index() -> anyhow::Result<()> {
    let index_path = Path::new("media_index");

    let json = include_str!("media_cache.json");
    let media: Vec<Media> = from_str(json)?;

    let mut index = Index::default();

    for m in &media {
        index.insert(m.clone());
    }

    let buf = rkyv::to_bytes::<_, 4096>(&index)?;

    fs::write(index_path, buf)?;

    Ok(())
}

pub fn search(query: &str) -> anyhow::Result<()> {
    let buf = include_bytes!("../media_index");

    let index = unsafe { rkyv::archived_root::<Index>(buf) };

    let lev_automaton_builder = levenshtein_automata::LevenshteinAutomatonBuilder::new(1, true);

    let dfa = lev_automaton_builder.build_dfa(query);

    for key in index.refs.keys() {
        let mut state = dfa.initial_state();

        for &b in key.as_bytes() {
            state = dfa.transition(state, b);
        }

        match dfa.distance(state) {
            levenshtein_automata::Distance::Exact(_) => console_log!("{:?}", key),
            levenshtein_automata::Distance::AtLeast(_) => (),
        }
    }

    Ok(())
}
