use wasm_bindgen::prelude::*;

use std::collections::{BTreeMap, HashSet};

mod characters;
mod media;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

pub(crate) fn normalize_text(text: &str) -> String {
    unidecode::unidecode(&text.to_lowercase())
}

pub(crate) fn tokenizer(text: String) -> Vec<String> {
    let words = text.split_whitespace().map(normalize_text);

    words.collect::<Vec<String>>()
}

#[derive(rkyv::Archive, rkyv::Serialize)]
pub(crate) struct Index<T> {
    pub(crate) refs: BTreeMap<String, HashSet<u32>>,
    pub(crate) data: Vec<T>,
}

pub(crate) trait Fields {
    fn fields(&self) -> Vec<String>;
}

pub(crate) trait Insert<T> {
    fn insert(&mut self, item: &T);
}

impl<T> Index<T> {
    pub(crate) fn default() -> Self {
        Self {
            refs: BTreeMap::new(),
            data: Vec::new(),
        }
    }
}

impl<T> Insert<T> for Index<T>
where
    T: Fields + Clone,
{
    fn insert(&mut self, item: &T) -> () {
        let i = self.data.len();

        let combined = item.fields();

        for s in &combined {
            let terms = tokenizer((*s).clone());

            for term in terms {
                self.refs
                    .entry(term)
                    .or_insert_with(HashSet::new)
                    .insert(i as u32);
            }
        }

        self.data.push(item.clone());
    }
}

pub(crate) fn create<T>(items: Vec<T>) -> Result<Vec<u8>, JsError>
where
    Index<T>: Insert<T>,
    T: Clone
        + rkyv::Serialize<
            rkyv::ser::serializers::CompositeSerializer<
                rkyv::ser::serializers::AlignedSerializer<rkyv::AlignedVec>,
                rkyv::ser::serializers::FallbackScratch<
                    rkyv::ser::serializers::HeapScratch<8192>,
                    rkyv::ser::serializers::AllocScratch,
                >,
                rkyv::ser::serializers::SharedSerializeMap,
            >,
        >,
{
    let mut index = Index::<T>::default();

    for item in &items {
        index.insert(item);
    }

    let buf = rkyv::to_bytes::<_, 8192>(&index)?;

    Ok(buf.to_vec())
}

// pub fn search<T>(query: &str, index_file: &[u8]) -> Result<Vec<T>, JsError>
// where
//     T: rkyv::Archive,
// {
//     let tokens = tokenizer(query.to_string());

//     let index = unsafe { rkyv::archived_root::<Index<T>>(index_file) };

//     let lev_automaton_builder = levenshtein_automata::LevenshteinAutomatonBuilder::new(2, true);

//     let mut results = HashSet::<u32>::new();

//     for token in tokens {
//         let dfa = lev_automaton_builder.build_dfa(&token);

//         for key in index.refs.keys() {
//             let mut state = dfa.initial_state();

//             for &b in key.as_bytes() {
//                 state = dfa.transition(state, b);
//             }

//             if let levenshtein_automata::Distance::Exact(_) = dfa.distance(state) {
//                 if let Some(refs) = index.refs.get(key) {
//                     results.extend(refs.iter());
//                 }
//             }
//         }
//     }

//     let mut t: Vec<&Archived<T>> = results
//         .iter()
//         .filter_map(|i| {
//             let archived = index.data.get(*i as usize);
//             archived
//         })
//         .collect();

//     // t.sort_by(|a, b| a.popularity.cmp(&b.popularity));

//     let tt: Vec<T> = t
//         .into_iter()
//         .take(25)
//         .filter_map(|archived| {
//             let character: Option<T> = archived.deserialize(&mut Infallible).ok();
//             character
//         })
//         .collect();

//     Ok(tt)
// }
