use crate::{create, tokenizer, Fields, Index};
use hashbrown::HashMap;
use rkyv::{Deserialize, Infallible};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, serde::Deserialize, Clone)]
pub struct Media {
    #[wasm_bindgen(getter_with_clone)]
    pub id: String,
    #[wasm_bindgen(getter_with_clone)]
    pub title: Vec<String>,
    pub popularity: u32,
}

struct Item<'a> {
    archived: Option<&'a ArchivedMedia>,
    document: Option<Media>,
    popularity: &'a u32,
    tokens_matched: u8,
    score: u8,
}

impl Fields for Media {
    fn fields(&self) -> Vec<String> {
        self.title.clone()
    }
}

#[wasm_bindgen]
impl Media {
    #[wasm_bindgen(constructor)]
    pub fn create(id: String, title: Vec<String>, popularity: u32) -> Media {
        Media {
            id,
            title,
            popularity,
        }
    }
}

#[wasm_bindgen]
pub fn create_media_index(json: &str) -> Result<Vec<u8>, JsError> {
    let items: Vec<Media> = serde_json::from_str(json)?;
    create(items)
}

#[wasm_bindgen]
pub fn search_media(
    query: &str,
    index_file: Option<Vec<u8>>,
    extra: Option<Vec<Media>>,
) -> Result<Vec<Media>, JsError> {
    let tokens = tokenizer(query.to_string());

    let lev_automaton_builder = levenshtein_automata::LevenshteinAutomatonBuilder::new(1, true);

    let index = index_file
        .as_ref()
        .map(|index_file| unsafe { rkyv::archived_root::<Index<Media>>(index_file) });

    let exrta_index = extra.as_ref().map(|extra| {
        let mut index = Index::<Media>::default();

        for media in extra {
            index.insert(media);
        }

        index
    });

    let mut items = HashMap::<String, Item>::new();

    let dias: Vec<_> = tokens
        .iter()
        .map(|token| lev_automaton_builder.build_dfa(token))
        .collect();

    if let Some(index) = index {
        for key in index.refs.keys() {
            for dfa in &dias {
                let mut state = dfa.initial_state();

                for &b in key.as_bytes() {
                    state = dfa.transition(state, b);
                }

                if let levenshtein_automata::Distance::Exact(score) = dfa.distance(state) {
                    if let Some(refs) = index.refs.get(key) {
                        for r in refs.iter() {
                            let archived = index.data.get(*r as usize).unwrap();

                            let item = items.entry(archived.id.to_string()).or_insert(Item {
                                archived: Some(archived),
                                document: None,
                                popularity: &archived.popularity,
                                tokens_matched: 0,
                                score,
                            });

                            if score == 0 {
                                item.tokens_matched += 1;
                            }

                            item.score = item.score.min(score);
                        }
                    }
                }
            }
        }
    }

    if let Some(index) = &exrta_index {
        for key in index.refs.keys() {
            for dfa in &dias {
                let mut state = dfa.initial_state();

                for &b in key.as_bytes() {
                    state = dfa.transition(state, b);
                }

                if let levenshtein_automata::Distance::Exact(score) = dfa.distance(state) {
                    if let Some(refs) = index.refs.get(key) {
                        for r in refs.iter() {
                            let document = index.data.get(*r as usize).unwrap();

                            let item = items.entry(document.id.clone()).or_insert(Item {
                                archived: None,
                                document: Some(document.clone()),
                                popularity: &document.popularity,
                                tokens_matched: 0,
                                score,
                            });

                            if score == 0 {
                                item.tokens_matched += 1;
                            }

                            item.score = item.score.min(score);
                        }
                    }
                }
            }
        }
    }

    let mut results: Vec<Item> = items.into_iter().map(|(_, item)| item).collect();

    results.sort_by(|a, b| {
        b.tokens_matched
            .cmp(&a.tokens_matched)
            .then_with(|| a.score.cmp(&b.score))
            .then_with(|| b.popularity.cmp(a.popularity))
    });

    let deserialized: Vec<Media> = results
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
