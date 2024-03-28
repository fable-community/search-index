use std::collections::HashSet;

use crate::{create, normalize_text, tokenizer, Fields, Index};

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

#[wasm_bindgen]
#[derive(Clone)]
pub struct CharacterResult {
    pub score: u32,
    #[wasm_bindgen(getter_with_clone)]
    pub character: Character,
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
pub fn search_characters(query: &str, index_file: &[u8]) -> Result<Vec<CharacterResult>, JsError> {
    let tokens = tokenizer(query.to_string());

    let index = unsafe { rkyv::archived_root::<Index<Character>>(index_file) };

    let lev_automaton_builder = levenshtein_automata::LevenshteinAutomatonBuilder::new(2, true);

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

    let normalized_query = normalize_text(query).into_bytes();

    let mut t: Vec<CharacterResult> = results
        .iter()
        .filter_map(|i| {
            let archived = index.data.get(*i as usize)?;
            let character: Character = archived.deserialize(&mut Infallible).ok()?;

            let score = character
                .fields()
                .iter()
                .map(|s| {
                    let normalized = normalize_text(s);
                    triple_accel::levenshtein(normalized.as_bytes(), &normalized_query)
                })
                .min()?;

            Some(CharacterResult { score, character })
        })
        .collect();

    t.sort_by(|a, b| a.score.cmp(&b.score));

    let mut tt: Vec<CharacterResult> = t.into_iter().take(25).collect();

    tt.sort_by(|a, b| b.character.popularity.cmp(&a.character.popularity));

    Ok(tt)
}
