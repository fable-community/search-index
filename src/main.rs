use std::time::Instant;

use serde_json::from_str;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

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

// #[derive(serde::Serialize, serde::Deserialize, Clone)]
// struct Media {
//     id: String,
//     title: Vec<String>,
//     popularity: u32,
// }

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Default)]
pub struct CharactersIndex {
    refs: BTreeMap<String, Vec<usize>>,
    data: Vec<Character>,
}

impl CharactersIndex {
    fn insert(&mut self, character: Character) -> () {
        let i = self.data.len();

        let mut combined = Vec::new();

        combined.extend(&character.name);
        combined.extend(&character.media_title);

        for s in &combined {
            let terms = tokenizer((*s).clone());

            for term in terms {
                self.refs.entry(term).or_insert_with(Vec::new).push(i);
            }
        }

        self.data.push(character);
    }
}

fn normalize_text(text: &str) -> String {
    unidecode::unidecode(&text.to_lowercase())
}

fn tokenizer(text: String) -> Vec<String> {
    let words = text.split_whitespace().map(normalize_text);

    words.collect::<Vec<String>>()
}

fn create_character_index() -> anyhow::Result<()> {
    let index_path = Path::new("characters_index");

    let index_creation = Instant::now();

    let json = include_str!("characters_cache.json");
    let characters: Vec<Character> = from_str(json)?;

    let mut index = CharactersIndex::default();

    for character in &characters {
        index.insert(character.clone());
    }

    let buf = rkyv::to_bytes::<_, 4096>(&index)?;

    fs::write(index_path, buf)?;

    println!("Index creation time: {:?}", index_creation.elapsed());

    Ok(())
}

fn search_character_index() -> anyhow::Result<()> {
    let index_path = Path::new("characters_index");

    if !index_path.exists() {
        create_character_index()?;
    }

    let buf = fs::read(index_path)?;

    let index = unsafe { rkyv::archived_root::<CharactersIndex>(&buf) };

    let lev_automaton_builder = levenshtein_automata::LevenshteinAutomatonBuilder::new(1, true);

    let dfa = lev_automaton_builder.build_dfa("luka");

    for key in index.refs.keys() {
        let mut state = dfa.initial_state();

        for &b in key.as_bytes() {
            state = dfa.transition(state, b);
        }

        match dfa.distance(state) {
            levenshtein_automata::Distance::Exact(_) => println!("{:?}", key),
            levenshtein_automata::Distance::AtLeast(_) => (),
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    search_character_index()?;
    Ok(())
}
