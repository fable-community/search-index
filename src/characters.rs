use crate::{create, tokenizer, Fields, Index, Insert};
use hashbrown::HashMap;
use rkyv::{Deserialize, Infallible};
use wasm_bindgen::prelude::*;

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
    #[wasm_bindgen(getter_with_clone)]
    pub role: String,
}

struct Item<'a> {
    archived: Option<&'a ArchivedCharacter>,
    document: Option<Character>,
    popularity: &'a u32,
    score: u8,
}

impl Fields for Character {
    fn fields(&self) -> Vec<String> {
        [self.name.clone(), self.media_title.clone()].concat()
    }
}

#[wasm_bindgen]
impl Character {
    #[wasm_bindgen(constructor)]
    pub fn create(
        id: String,
        name: Vec<String>,
        media_title: Vec<String>,
        popularity: u32,
        rating: u32,
        role: String,
    ) -> Character {
        Character {
            id,
            name,
            media_title,
            popularity,
            rating,
            role,
        }
    }
}

#[wasm_bindgen]
pub fn create_characters_index(json: &str) -> Result<Vec<u8>, JsError> {
    let items: Vec<Character> = serde_json::from_str(json)?;
    create(items)
}

#[wasm_bindgen]
pub fn search_characters(
    query: &str,
    index_file: Option<Vec<u8>>,
    extra: Option<Vec<Character>>,
) -> Result<Vec<Character>, JsError> {
    let tokens = tokenizer(query.to_string());

    let lev_automaton_builder = levenshtein_automata::LevenshteinAutomatonBuilder::new(1, true);

    let index = index_file
        .as_ref()
        .map(|index_file| unsafe { rkyv::archived_root::<Index<Character>>(index_file) });

    let exrta_index = extra.as_ref().map(|extra| {
        let mut index = Index::<Character>::default();

        for character in extra {
            index.insert(character);
        }

        index
    });

    let mut items = HashMap::<String, Item>::new();

    for token in &tokens {
        let dfa = lev_automaton_builder.build_dfa(&token);

        if let Some(index) = index {
            for key in index.refs.keys() {
                let mut state = dfa.initial_state();

                for &b in key.as_bytes() {
                    state = dfa.transition(state, b);
                }

                if let levenshtein_automata::Distance::Exact(score) = dfa.distance(state) {
                    if let Some(refs) = index.refs.get(key) {
                        items.extend(refs.iter().filter_map(|i| {
                            let archived = index.data.get(*i as usize)?;

                            Some((
                                archived.id.to_string(),
                                Item {
                                    archived: Some(archived),
                                    document: None,
                                    popularity: &archived.popularity,
                                    score,
                                },
                            ))
                        }));
                    }
                }
            }
        }

        if let Some(index) = &exrta_index {
            for key in index.refs.keys() {
                let mut state = dfa.initial_state();

                for &b in key.as_bytes() {
                    state = dfa.transition(state, b);
                }

                if let levenshtein_automata::Distance::Exact(score) = dfa.distance(state) {
                    if let Some(refs) = index.refs.get(key) {
                        items.extend(refs.iter().filter_map(|i| {
                            let document = index.data.get(*i as usize)?;

                            Some((
                                document.id.to_string(),
                                Item {
                                    archived: None,
                                    document: Some(document.clone()),
                                    popularity: &document.popularity,
                                    score,
                                },
                            ))
                        }));
                    }
                }
            }
        }
    }

    let mut results: Vec<Item> = items.into_iter().map(|(_, item)| item).collect();

    results.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .then_with(|| b.popularity.cmp(&a.popularity))
    });

    let deserialized: Vec<Character> = results
        .into_iter()
        .take(25)
        .filter_map(|item| {
            if let Some(archived) = item.archived {
                archived.deserialize(&mut Infallible).ok()
            } else {
                item.document
            }
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
                if role != &character.role.to_string() {
                    return None;
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
