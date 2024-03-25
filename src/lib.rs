use std::path::Path;
use std::time::Instant;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::Index;

pub fn search_characters(query_string: &str) -> tantivy::Result<()> {
    let start_time = Instant::now();

    let index_path = Path::new("characters_index");

    let index = Index::open_in_dir(index_path)?;

    let schema = index.schema();

    let name = schema.get_field("name")?;
    let media_title = schema.get_field("media_title")?;

    // Create a query parser
    let query_parser = QueryParser::for_index(&index, vec![name, media_title]);

    // Perform the search
    let query = query_parser.parse_query(query_string)?;
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    // Print the search results
    for (score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        println!(
            "{:?} score:{:?}",
            schema.to_named_doc(&retrieved_doc),
            score
        );
    }

    println!("Total time elapsed: {:?}", start_time.elapsed());

    Ok(())
}

pub fn search_media(query_string: &str) -> tantivy::Result<()> {
    let start_time = Instant::now();

    let index_path = Path::new("media_index");

    let index = Index::open_in_dir(index_path)?;

    let schema = index.schema();

    let title = schema.get_field("title")?;

    // Create a query parser
    let query_parser = QueryParser::for_index(&index, vec![title]);

    // Perform the search
    let query = query_parser.parse_query(query_string)?;
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    // Print the search results
    for (score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        println!(
            "{:?} score:{:?}",
            schema.to_named_doc(&retrieved_doc),
            score
        );
    }

    println!("Total time elapsed: {:?}", start_time.elapsed());

    Ok(())
}
