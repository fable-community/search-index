use crate::{create, tokenizer, Fields, Index};
use hashbrown::HashSet;
use rkyv::{Deserialize, Infallible};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, serde::Deserialize, PartialEq, Clone, Copy,
)]
pub enum CharacterRole {
    MAIN,
    SUPPORTING,
    BACKGROUND,
}

#[wasm_bindgen]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, serde::Deserialize, Clone)]
pub struct Character {
    #[wasm_bindgen(getter_with_clone)]
    pub id: String,
    #[wasm_bindgen(getter_with_clone)]
    pub name: Vec<String>,
    #[serde(rename = "mediaTitle")]
    #[wasm_bindgen(getter_with_clone, js_name = mediaTitle)]
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

#[wasm_bindgen]
pub fn filter_characters(
    role: Option<String>,
    popularity_lesser: Option<u32>,
    popularity_greater: Option<u32>,
    rating: Option<u32>,
    index_file: &[u8],
) -> Result<Vec<Character>, JsError> {
    let index = unsafe { rkyv::archived_root::<Index<Character>>(index_file) };

    let results_as_archived: Vec<&ArchivedCharacter> = index
        .data
        .iter()
        .filter_map(|character| {
            if let Some(rating) = rating {
                if character.rating != rating {
                    return None;
                }
            }

            if let Some(popularity_lesser) = popularity_lesser {
                if character.popularity < popularity_lesser {
                    return None;
                }
            }

            if let Some(popularity_greater) = popularity_greater {
                if character.popularity < popularity_greater {
                    return None;
                }
            }

            if let Some(role) = &role {
                match character.role {
                    ArchivedCharacterRole::MAIN => {
                        if role != "MAIN" {
                            return None;
                        }
                    }
                    ArchivedCharacterRole::SUPPORTING => {
                        if role != "SUPPORTING" {
                            return None;
                        }
                    }
                    ArchivedCharacterRole::BACKGROUND => {
                        if role != "BACKGROUND" {
                            return None;
                        }
                    }
                }
            }

            Some(character)
        })
        .collect();

    let deserialized: Vec<Character> = results_as_archived
        .into_iter()
        .filter_map(|archived| {
            let item: Option<Character> = archived.deserialize(&mut Infallible).ok();
            item
        })
        .collect();

    Ok(deserialized)
}
