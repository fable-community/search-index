use wasm_bindgen::prelude::*;

use crate::{
    console_log, {tokenizer, Index, Insert},
};

#[derive(
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, serde::Serialize, serde::Deserialize, Clone,
)]
struct Media {
    id: String,
    title: Vec<String>,
    popularity: u32,
}

impl Insert<Media> for Index<Media> {
    fn insert(&mut self, item: Media) -> () {
        let i = self.data.len();

        let mut combined = Vec::new();

        combined.extend(&item.title);

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
pub fn create_media_index(media_json: &str) -> Result<Vec<u8>, JsError> {
    let mut index = Index::<Media>::default();

    let media: Vec<Media> = serde_json::from_str(media_json)?;

    for m in &media {
        index.insert(m.clone());
    }

    let buf = rkyv::to_bytes::<_, 4096>(&index)?;

    Ok(buf.to_vec())
}

#[wasm_bindgen]
pub fn search_media(query: &str, index_file: &[u8]) -> Result<(), JsError> {
    let index = unsafe { rkyv::archived_root::<Index<Media>>(index_file) };

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
