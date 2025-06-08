use async_trait::async_trait;
use document::Document; // Reverted to document::Document
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

// Define a custom error type for VectorDb operations
#[derive(Debug)]
pub enum VectorDbError {
    NotImplemented,
    ConnectionError(String),
    OperationFailed(String),
    // Add other specific error types as needed
}

impl fmt::Display for VectorDbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VectorDbError::NotImplemented => write!(f, "Operation not implemented"),
            VectorDbError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            VectorDbError::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
        }
    }
}

impl Error for VectorDbError {} // Allow VectorDbError to be treated as a generic Error


#[async_trait]
pub trait VectorDb {
    /// Creates the vector database or collection.
    fn create(&mut self) -> Result<(), VectorDbError>;
    async fn async_create(&mut self) -> Result<(), VectorDbError>;

    /// Checks if a specific document already exists in the database.
    fn doc_exists(&self, document: &Document) -> Result<bool, VectorDbError>;
    async fn async_doc_exists(&self, document: &Document) -> Result<bool, VectorDbError>;

    /// Checks if a document with the given name exists.
    fn name_exists(&self, name: &str) -> Result<bool, VectorDbError>;
    async fn async_name_exists(&self, name: &str) -> Result<bool, VectorDbError>;

    /// Checks if a document with the given ID exists.
    fn id_exists(&self, id: &str) -> Result<bool, VectorDbError>;

    /// Inserts documents into the database.
    fn insert(&mut self, documents: &[Document], filters: Option<HashMap<String, JsonValue>>) -> Result<(), VectorDbError>;
    async fn async_insert(&mut self, documents: &[Document], filters: Option<HashMap<String, JsonValue>>) -> Result<(), VectorDbError>;

    /// Indicates if the database supports an upsert operation.
    fn upsert_available(&self) -> bool {
        false
    }

    /// Upserts documents into the database.
    fn upsert(&mut self, documents: &[Document], filters: Option<HashMap<String, JsonValue>>) -> Result<(), VectorDbError>;
    async fn async_upsert(&mut self, documents: &[Document], filters: Option<HashMap<String, JsonValue>>) -> Result<(), VectorDbError>;

    /// Searches the database for documents matching the query.
    fn search(&self, query: &str, limit: u32, filters: Option<HashMap<String, JsonValue>>) -> Result<Vec<Document>, VectorDbError>;
    async fn async_search(&self, query: &str, limit: u32, filters: Option<HashMap<String, JsonValue>>) -> Result<Vec<Document>, VectorDbError>;

    /// Performs a vector-based search.
    fn vector_search(&self, query: &str, limit: u32) -> Result<Vec<Document>, VectorDbError>;

    /// Performs a keyword-based search.
    fn keyword_search(&self, query: &str, limit: u32) -> Result<Vec<Document>, VectorDbError>;

    /// Performs a hybrid search (vector + keyword).
    fn hybrid_search(&self, query: &str, limit: u32) -> Result<Vec<Document>, VectorDbError>;

    /// Drops the entire database or collection.
    fn drop_db(&mut self) -> Result<(), VectorDbError>;
    async fn async_drop_db(&mut self) -> Result<(), VectorDbError>;

    /// Checks if the database or collection currently exists.
    fn db_exists(&self) -> Result<bool, VectorDbError>;
    async fn async_db_exists(&self) -> Result<bool, VectorDbError>;

    /// Optimizes the database (e.g., indexing).
    fn optimize(&mut self) -> Result<(), VectorDbError>;

    /// Deletes the database or collection.
    fn delete(&mut self) -> Result<bool, VectorDbError>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
