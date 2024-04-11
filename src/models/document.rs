use actix_web::body::{MessageBody};
use serde::{Deserialize, Serialize};
use crate::enums::doc_type::DOCTYPE;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub text: String,
    pub description: String,
    pub doc_type: DOCTYPE,
    pub path: Option<String>,
    pub file_names: Vec<String>,
}

impl Default for Document {
    fn default() -> Self {
        Document {
            id: String::new(),
            text: String::new(),
            description: String::new(),
            doc_type: DOCTYPE::NEWS,
            path: None,
            file_names: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocFile {
    pub name: String,
    pub path: String,
}