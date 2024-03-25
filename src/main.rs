use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs;
use std::path::Path;
use tantivy::schema::{Document, Schema, STORED, TEXT};
use tantivy::{doc, Index};

#[derive(Serialize, Deserialize, Debug)]
struct Character {
    id: String,
    name: Vec<String>,
    #[serde(rename = "mediaTitle")]
    media_title: Vec<String>,
    popularity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Media {
    id: String,
    title: Vec<String>,
    popularity: u32,
}

fn index_characters() -> tantivy::Result<()> {
    let mut schema_builder = Schema::builder();

    let name = schema_builder.add_text_field("name", TEXT | STORED);
    let media_title = schema_builder.add_text_field("media_title", TEXT | STORED);

    schema_builder.add_text_field("id", STORED);
    schema_builder.add_text_field("popularity", STORED);

    let schema = schema_builder.build();

    // Specify the directory where the index will be saved
    let index_path = Path::new("character_index");
    let index_exists = index_path.exists();

    if index_exists {
        return Ok(());
    }

    fs::create_dir_all(index_path).unwrap();

    println!("Creating a new characters index...");
    let index = Index::create_in_dir(index_path, schema.clone())?;

    // Add documents to the index if it's new
    if !index_exists {
        println!("Adding documents to the index...");
        // Load characters from JSON
        let json = include_str!("characters_directory.json");
        let characters: Vec<Character> = from_str(json).unwrap();

        let mut index_writer = index.writer(50_000_000)?; // 50mb
        for character in characters {
            let mut doc = Document::default();

            for _name in &character.name {
                doc.add_text(name, _name);
            }

            for _media_title in &character.media_title {
                doc.add_text(media_title, _media_title);
            }

            let _ = index_writer.add_document(doc);
        }

        println!("Committing documents to the index...");
        index_writer.commit()?;
    }

    Ok(())
}

fn index_media() -> tantivy::Result<()> {
    let mut schema_builder = Schema::builder();

    let title = schema_builder.add_text_field("title", TEXT | STORED);

    schema_builder.add_text_field("id", STORED);
    schema_builder.add_text_field("popularity", STORED);

    let schema = schema_builder.build();

    // Specify the directory where the index will be saved
    let index_path = Path::new("media_index");
    let index_exists = index_path.exists();

    if index_exists {
        return Ok(());
    }

    fs::create_dir_all(index_path).unwrap();

    println!("Creating a new media index...");
    let index = Index::create_in_dir(index_path, schema.clone())?;

    // Add documents to the index if it's new
    if !index_exists {
        println!("Adding documents to the index...");
        // Load characters from JSON
        let json = include_str!("media_directory.json");
        let media: Vec<Media> = from_str(json).unwrap();

        let mut index_writer = index.writer(50_000_000)?; // 50mb
        for _media in media {
            let mut doc = Document::default();

            for _title in &_media.title {
                doc.add_text(title, _title);
            }

            let _ = index_writer.add_document(doc);
        }

        println!("Committing documents to the index...");
        index_writer.commit()?;
    }

    Ok(())
}

fn main() -> tantivy::Result<()> {
    index_characters()?;
    index_media()?;
    Ok(())
}
