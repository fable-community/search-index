use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::path::Path;
use std::{fs, io};

#[derive(Serialize, Deserialize, Debug)]
enum CharacterRole {
    MAIN,
    SUPPORTING,
    BACKGROUND,
}

#[derive(Serialize, Deserialize, Debug)]
struct Character {
    id: String,
    name: Vec<String>,
    #[serde(rename = "mediaTitle")]
    media_title: Vec<String>,
    popularity: u32,
    rating: u32,
    role: CharacterRole,
}

#[derive(Serialize, Deserialize, Debug)]
struct Media {
    id: String,
    title: Vec<String>,
    popularity: u32,
}

fn index_characters() -> io::Result<()> {
    let index_path = Path::new("characters_index");
    let index_exists = index_path.exists();

    if index_exists {
        return Ok(());
    }

    let json = include_str!("characters_cache.json");
    let characters: Vec<Character> = from_str(json).unwrap();

    fs::create_dir_all(index_path).unwrap();

    Ok(())
}

fn index_media() -> io::Result<()> {
    let index_path = Path::new("media_index");
    let index_exists = index_path.exists();

    if index_exists {
        return Ok(());
    }

    let json = include_str!("media_cache.json");
    let media: Vec<Media> = from_str(json).unwrap();

    fs::create_dir_all(index_path).unwrap();

    Ok(())
}

fn main() -> io::Result<()> {
    index_characters()?;
    index_media()?;
    Ok(())
}
