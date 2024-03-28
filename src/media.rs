use std::collections::HashSet;

use crate::{create, tokenizer, Fields, Index};

use rkyv::{Deserialize, Infallible};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, serde::Deserialize, Clone)]
pub struct Media {
    pub id: String,
    pub title: Vec<String>,
    pub popularity: u32,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct MediaResult {
    pub score: u32,
    #[wasm_bindgen(getter_with_clone)]
    pub media: Media,
}

impl Fields for Media {
    fn fields(&self) -> Vec<String> {
        self.title.clone()
    }
}

#[wasm_bindgen]
pub fn create_media_index(json: &str) -> Result<Vec<u8>, JsError> {
    let items: Vec<Media> = serde_json::from_str(json)?;
    create(items)
}

#[wasm_bindgen]
pub fn search_media(query: &str, index_file: &[u8]) -> Result<Vec<Media>, JsError> {
    let tokens = tokenizer(query.to_string());

    let index = unsafe { rkyv::archived_root::<Index<Media>>(index_file) };

    let lev_automaton_builder = levenshtein_automata::LevenshteinAutomatonBuilder::new(1, true);

    let mut results = HashSet::<u32>::new();

    for token in tokens {
        let dfa = lev_automaton_builder.build_dfa(&token);

        for key in index.refs.keys() {
            let mut state = dfa.initial_state();

            for &b in key.as_bytes() {
                state = dfa.transition(state, b);
            }

            if let levenshtein_automata::Distance::Exact(_) = dfa.distance(state) {
                if let Some(refs) = index.refs.get(key) {
                    results.extend(refs.iter());
                }
            }
        }
    }

    let mut t: Vec<&ArchivedMedia> = results
        .iter()
        .filter_map(|i| {
            let archived = index.data.get(*i as usize);
            archived
        })
        .collect();

    t.sort_by(|a, b| b.popularity.cmp(&a.popularity));

    let tt: Vec<Media> = t
        .into_iter()
        .take(25)
        .filter_map(|archived| {
            let character: Option<Media> = archived.deserialize(&mut Infallible).ok();
            character
        })
        .collect();

    Ok(tt)
}
