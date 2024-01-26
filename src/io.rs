use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub document_id: String,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct IndexingInput {
    pub group_id: String,
    pub user_id: String,
    pub session_id: String,
    pub documents:Vec<Document>
}

#[derive(Serialize, Deserialize)]
pub struct IndexingOutput {
    pub group_id: String,
    pub user_id: String,
    pub session_id: String,
    pub save_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchInput {
    pub group_id: String,
    pub user_id: String,
    pub session_id: String,
    pub query: Query,
    pub top_k: u16,
}


#[derive(Serialize, Deserialize)]
pub struct SearchOutput {
    pub group_id: String,
    pub user_id: String,
    pub session_id: String,
    pub related_documents: Vec<Document>,
}



#[derive(Serialize, Deserialize)]
pub struct DeleteInput {
    pub group_id: String,
    pub user_id: String,
    pub session_id: String,
    pub document_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteOutput {
    pub group_id: String,
    pub user_id: String,
    pub session_id: String,
    pub deleted: bool,
}
