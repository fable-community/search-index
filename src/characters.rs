use wasm_bindgen::prelude::*;

use crate::{
    console_log, {tokenizer, Index, Insert},
};

#[derive(
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Copy,
)]
enum CharacterRole {
    MAIN,
    SUPPORTING,
    BACKGROUND,
}

#[derive(
    serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone,
)]
struct Character {
    id: String,
    name: Vec<String>,
    #[serde(rename = "mediaTitle")]
    media_title: Vec<String>,
    popularity: u32,
    rating: u32,
    role: CharacterRole,
}

impl Insert<Character> for Index<Character> {
    fn insert(&mut self, item: Character) -> () {
        let i = self.data.len();

        let mut combined = Vec::new();

        combined.extend(&item.name);
        combined.extend(&item.media_title);

        for s in &combined {
            let terms = tokenizer((*s).clone());

            for term in terms {
                self.refs.entry(term).or_insert_with(Vec::new).push(i);
            }
        }

        self.data.push(item);
    }
}

#[wasm_bindgen]
pub fn create_characters_index(characters_json: &str) -> Result<Vec<u8>, JsError> {
    let mut index = Index::<Character>::default();

    let characters: Vec<Character> = serde_json::from_str(characters_json)?;

    for character in &characters {
        index.insert(character.clone());
    }

    let buf = rkyv::to_bytes::<_, 4096>(&index)?;

    Ok(buf.to_vec())
}

#[wasm_bindgen]
pub fn search_characters(query: &str, index_file: &[u8]) -> Result<(), JsError> {
    let index = unsafe { rkyv::archived_root::<Index<Character>>(index_file) };

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
