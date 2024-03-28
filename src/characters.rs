use crate::{create, tokenizer, Fields, Index};
use hashbrown::HashSet;
use rkyv::{Deserialize, Infallible};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, serde::Deserialize, Clone)]
pub enum CharacterRole {
    MAIN,
    SUPPORTING,
    BACKGROUND,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, serde::Deserialize, Clone)]
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

impl Fields for Character {
    fn fields(&self) -> Vec<String> {
        [self.name.clone(), self.media_title.clone()].concat()
    }
}

#[wasm_bindgen]
pub fn create_characters_index(json: &str) -> Result<Vec<u8>, JsError> {
    let items: Vec<Character> = serde_json::from_str(json)?;
    create(items)
}

#[wasm_bindgen]
pub fn search_characters(query: &str, index_file: &[u8]) -> Result<Vec<Character>, JsError> {
    let tokens = tokenizer(query.to_string());

    let index = unsafe { rkyv::archived_root::<Index<Character>>(index_file) };

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

    let mut results_as_archived: Vec<(&ArchivedCharacter, &u8)> = results
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

    let deserialized: Vec<Character> = results_as_archived
        .into_iter()
        .take(25)
        .filter_map(|(archived, _)| {
            let item: Option<Character> = archived.deserialize(&mut Infallible).ok();
            item
        })
        .collect();

    Ok(deserialized)
}
