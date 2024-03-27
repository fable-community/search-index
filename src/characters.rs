use crate::{create_index_fn, normalize_text, searchable, tokenizer, Index, Insert};
use rkyv::{Deserialize, Infallible};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, serde::Serialize, serde::Deserialize, Clone,
)]
pub enum CharacterRole {
    MAIN,
    SUPPORTING,
    BACKGROUND,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(
    serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone,
)]
pub struct Character {
    pub id: String,
    pub name: Vec<String>,
    #[serde(rename = "mediaTitle")]
    #[wasm_bindgen(js_name = mediaTitle)]
    pub media_title: Vec<String>,
    pub popularity: u32,
    pub rating: u32,
    pub role: CharacterRole,
}

searchable!(Character, name, media_title);

create_index_fn!(Character, create_characters_index);

#[wasm_bindgen]
pub fn search_characters(query: &str, index_file: &[u8]) -> Result<Vec<Character>, JsError> {
    let tokens = tokenizer(query.to_string());

    let index = unsafe { rkyv::archived_root::<Index<Character>>(index_file) };
    let lev_automaton_builder = levenshtein_automata::LevenshteinAutomatonBuilder::new(1, true);

    let mut combined_results = HashSet::<u32>::new();

    for token in tokens {
        let dfa = lev_automaton_builder.build_dfa(&token);

        for key in index.refs.keys() {
            let mut state = dfa.initial_state();

            for &b in key.as_bytes() {
                state = dfa.transition(state, b);
            }

            if let levenshtein_automata::Distance::Exact(_) = dfa.distance(state) {
                if let Some(refs) = index.refs.get(key) {
                    combined_results.extend(refs.iter());
                }
            }
        }
    }

    // TODO score results
    // TODO sort results
    // TODO limit results

    // TODO allow inserting new items to index before search

    Ok(Vec::default())

    // Ok(set
    //     .iter()
    //     .filter_map(|i| {
    //         let archived = index.data.get(*i as usize)?;
    //         let character: Option<Character> = archived.deserialize(&mut Infallible).ok();
    //         character
    //     })
    //     .collect())
}
