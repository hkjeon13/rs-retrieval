
use axum::{
    routing::{post},
    http::StatusCode,
    Json, Router,
};

mod io_utils;
use io_utils::{IndexingInput, SearchInput, DeleteInput, IndexingOutput, SearchOutput, DeleteOutput};

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

async fn indexing(
    Json(payload): Json<IndexingInput>,
) -> (StatusCode, Json<IndexingOutput>) {

    let output = IndexingOutput {
        group_id: payload.group_id,
        user_id: payload.user_id,
        session_id: payload.session_id,
        save_path: "save_path".to_string(),
    };

    (StatusCode::CREATED, Json(output))
}

async fn search(
    Json(payload): Json<SearchInput>,
) -> (StatusCode, Json<SearchOutput>) {

    let output = SearchOutput {
        group_id: payload.group_id,
        user_id: payload.user_id,
        session_id: payload.session_id,
        related_documents: vec![],
    };

    (StatusCode::CREATED, Json(output))
}

async fn delete(
    Json(payload): Json<DeleteInput>,
) -> (StatusCode, Json<DeleteOutput>) {

    let output = DeleteOutput {
        group_id: payload.group_id,
        user_id: payload.user_id,
        session_id: payload.session_id,
        deleted: true,
    };

    (StatusCode::CREATED, Json(output))
}

