use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use io::{IndexingInput, SearchInput, DeleteInput};

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/indexing", post(create_user)) // indexing
        .route("/search", post(create_user)) // search
        .route("/delete", post(create_user)); // delete

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn indexing(
    Json(payload): Json<IndexingInput>,
) -> (StatusCode, Json<IndexingOutput>) {

    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

async fn search(
    Json(payload): Json<SearchInput>,
) -> (StatusCode, Json<SearchOutput>) {

    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

async fn delete(
    Json(payload): Json<DeleteInput>,
) -> (StatusCode, Json<DeleteOutput>) {

    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

