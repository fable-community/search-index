use crate::{create_index_fn, searchable, tokenizer, Index, Insert};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, serde::Serialize, serde::Deserialize, Clone,
)]
struct Media {
    pub id: String,
    pub title: Vec<String>,
    pub popularity: u32,
}

searchable!(Media, title);

create_index_fn!(Media, create_media_index);

// #[wasm_bindgen]
// pub fn search_media(query: &str, index_file: &[u8]) -> Result<Vec<Media>,, JsError> {
//     let index = unsafe { rkyv::archived_root::<Index<Media>>(index_file) };

//     let lev_automaton_builder = levenshtein_automata::LevenshteinAutomatonBuilder::new(1, true);

//     let dfa = lev_automaton_builder.build_dfa(query);

//     for key in index.refs.keys() {
//         let mut state = dfa.initial_state();

//         for &b in key.as_bytes() {
//             state = dfa.transition(state, b);
//         }

//         match dfa.distance(state) {
//             levenshtein_automata::Distance::Exact(_) => console_log!("{:?}", key),
//             levenshtein_automata::Distance::AtLeast(_) => (),
//         }
//     }

//     Ok(())
// }
