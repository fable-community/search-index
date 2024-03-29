use crate::{create, tokenizer, Fields, Index};
use hashbrown::HashSet;
use rkyv::{Deserialize, Infallible};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, serde::Deserialize, Clone)]
pub struct Media {
    #[wasm_bindgen(getter_with_clone)]
    pub id: String,
    #[wasm_bindgen(getter_with_clone)]
    pub title: Vec<String>,
    pub popularity: u32,
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

    let mut results = HashSet::<(&u32, u8)>::new();

    for token in tokens {
        let dfa = lev_automaton_builder.build_dfa(&token);

        for key in index.refs.keys() {
            let mut state = dfa.initial_state();

            for &b in key.as_bytes() {
                state = dfa.transition(state, b);
            }

            if let levenshtein_automata::Distance::Exact(s) = dfa.distance(state) {
                if let Some(refs) = index.refs.get(key) {
                    results.extend(refs.iter().map(|r| (r, s)));
                }
            }
        }
    }

    let mut results_as_archived: Vec<(&ArchivedMedia, &u8)> = results
        .iter()
        .filter_map(|(i, s)| {
            let archived = index.data.get(**i as usize)?;
            Some((archived, s))
        })
        .collect();

    results_as_archived.sort_by(|(a_item, a_score), (b_item, b_score)| {
        a_score
            .cmp(b_score)
            .then_with(|| b_item.popularity.cmp(&a_item.popularity))
    });

    let deserialized: Vec<Media> = results_as_archived
        .into_iter()
        .take(25)
        .filter_map(|(archived, _)| {
            let item: Option<Media> = archived.deserialize(&mut Infallible).ok();
            item
        })
        .collect();

    Ok(deserialized)
}
