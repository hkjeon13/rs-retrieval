
use axum::{
    routing::{post},
    http::StatusCode,
    Json, Router,
};

mod io_utils;
mod bm25;
use io_utils::{IndexingInput, SearchInput, DeleteInput, IndexingOutput, SearchOutput, DeleteOutput, Document};
use tokenizers::tokenizer::{Result, Tokenizer};
use tokenizers::tokenizer::{EncodeInput, TruncationParams, TruncationStrategy,TruncationDirection, PaddingParams, PaddingStrategy};
use bm25::{BM25};
use std::fs;
use std::path::{Path, PathBuf};
extern crate mecab;
use std::collections::{HashMap, HashSet};
use serde_json;
use mecab::Tagger;

#[tokio::main]
async fn main() {

    let app = Router::new()
        // indexing
        .route("/indexing", post(indexing))
        // search
        .route("/search", post(search))
        // delete
        .route("/delete", post(delete));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn chunk(tokenizer: &Tokenizer, text: String) -> Vec<String> {
    let encoding = tokenizer.encode(
        EncodeInput::Single(text.into()),
        false
    ).unwrap();

    let mut output = encoding.get_overflowing().iter().map(|x| x.get_ids()).collect::<Vec<&[u32]>>();
    output.insert(0, encoding.get_ids());
    tokenizer.decode_batch(&output, true).unwrap()
}

fn get_nouns_from_texts(tagger: &Tagger, texts:Vec<String>) -> Vec<Vec<String>> {
    texts.into_iter().map(|text| parse_nouns(tagger, text)).collect()
}


fn parse_nouns(tagger: &Tagger, text: String) -> Vec<String> {
    tagger.parse_str(&*text)
        .split('\n')
        .filter_map(|line| {
            let mut parts = line.split('\t');
            match (parts.next(), parts.next()) {
                (Some(word), Some(tags)) => {
                    let tag = tags.split(',').next()?;
                    if tag.starts_with('N') && word.len() > 1 {
                        Some(word.to_string())
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }).collect()
}

async fn indexing(
    Json(payload): Json<IndexingInput>,
) -> (StatusCode, Json<IndexingOutput>) {
    let save_dir = format!("indexing_files/{}/", payload.group_id);
    let save_path = format!("{}bm25.json", save_dir.as_str());
    let mut tokenizer = Tokenizer::from_file("model/tokenizer.json").unwrap();
    let mut tagger = Tagger::new("");

    let _ = tokenizer.with_truncation(Some(TruncationParams{
        max_length: 512,
        strategy: TruncationStrategy::LongestFirst,
        stride:32,
        direction: TruncationDirection::Right,
    }));

    tokenizer.with_padding(Some(PaddingParams::default()));
    let chunked_documents = payload.documents.iter().map(|doc|
        (doc.document_id.to_string(), chunk(&tokenizer, doc.text.clone()))
    ).collect::<Vec<(String, Vec<String>)>>();


    let chunked_documents_tokenized = chunked_documents.iter().map(|(_, doc)|
        get_nouns_from_texts(&tagger, doc.clone())
    ).collect::<Vec<Vec<Vec<String>>>>();

    let mut bm25 = match Path::new(&save_path).exists(){
        true => BM25::load(save_path.to_string()),
        false => BM25::new(),
    };
    let mut id_to_doc = HashMap::new();
    println!("The number of pre-indexed tokens: {:?}", bm25.index_map.len());
    for ((doc_id, chunk_text), chunked_document) in chunked_documents.iter().zip(chunked_documents_tokenized.iter()) {
        for (index, chunk) in chunked_document.iter().enumerate(){
            let chunk_id = format!("{}@{}", doc_id, index);
            bm25.add_document(chunk_id.to_string(), chunk.clone());
            id_to_doc.insert(chunk_id.to_string(), chunk_text[index].clone());
        }
    }

    bm25.freeze();

    fs::create_dir_all(save_dir.as_str()).unwrap();
    bm25.save(save_path.to_string());
    //save chunked_documents
    let chunked_documents_path = format!("{}chunked_documents.json", save_dir);
    let json_file = serde_json::to_string(&id_to_doc).unwrap();
    std::fs::write(chunked_documents_path, json_file).expect("Unable to write file");

    let output = IndexingOutput {
        group_id: payload.group_id,
        user_id: payload.user_id,
        session_id: payload.session_id,
        save_path: save_path.to_string(),
    };

    (StatusCode::OK, Json(output))
}

async fn search(
    Json(payload): Json<SearchInput>,
) -> (StatusCode, Json<SearchOutput>) {

    let save_path = format!("indexing_files/{}/bm25.json", payload.group_id);
    let bm25 = BM25::load(save_path.to_string());
    let tagger = Tagger::new("");
    let tokens = parse_nouns(&tagger, payload.query.text);

    let related_documents = bm25.search(
        tokens,
        payload.top_k as usize
    );

    let output = SearchOutput {
        group_id: payload.group_id,
        user_id: payload.user_id,
        session_id: payload.session_id,
        related_documents: related_documents.iter().map(|(id, _)|
            Document {
                document_id: id.to_string(),
                text: id.to_string(),
            }
        ).collect::<Vec<Document>>()
    };

    (StatusCode::OK, Json(output))
}

async fn delete(
    Json(payload): Json<DeleteInput>,
) -> (StatusCode, Json<DeleteOutput>) {

    let dir = format!("indexing_files/{}/", payload.group_id);
    let target = format!("{}bm25.json", dir);

    let mut bm25 = match Path::new(&target).exists(){
        true => BM25::load(target.to_string()),
        false => BM25::new(),
    };

    bm25.delete_document(payload.document_id);

    if bm25.index_map.len() == 0 {
        fs::remove_dir_all(dir).unwrap();
    } else {
        bm25.save(target.to_string());
    }

    let output = DeleteOutput {
        group_id: payload.group_id,
        user_id: payload.user_id,
        session_id: payload.session_id,
        deleted: true,
    };

    (StatusCode::OK, Json(output))
}

