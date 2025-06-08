// Import the Document struct from the document crate
use document::Document; // Reverted

#[derive(Debug, Clone, PartialEq)]
pub struct DocumentKnowledgeBase {
    pub documents: Vec<Document>,
}

impl DocumentKnowledgeBase {
    /// Creates a new DocumentKnowledgeBase with the given documents.
    pub fn new(documents: Vec<Document>) -> Self {
        DocumentKnowledgeBase { documents }
    }

    /// Provides an iterator that yields a list containing each document one by one.
    pub fn document_lists_iter(&self) -> impl Iterator<Item = Vec<Document>> + '_ {
        self.documents.iter().map(|doc| vec![doc.clone()])
    }

    /// Provides an iterator over references to the documents.
    pub fn iter_documents(&self) -> impl Iterator<Item = &Document> + '_ {
        self.documents.iter()
    }

    /// Adds a document to the knowledge base.
    pub fn add_document(&mut self, document: Document) {
        self.documents.push(document);
    }

    /// Returns the number of documents in the knowledge base.
    pub fn len(&self) -> usize {
        self.documents.len()
    }

    /// Returns true if the knowledge base contains no documents.
    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use document::Document; // Reverted
    use std::collections::HashMap;

    fn create_test_document(id: &str, content: &str) -> Document {
        Document {
            id: Some(id.to_string()),
            content: content.to_string(),
            name: Some(format!("Doc {}", id)),
            meta_data: HashMap::new(),
            usage: None,
            reranking_score: None,
        }
    }

    #[test]
    fn test_new_knowledge_base() {
        let doc1 = create_test_document("1", "Content 1");
        let doc2 = create_test_document("2", "Content 2");
        let documents = vec![doc1.clone(), doc2.clone()];

        let kb = DocumentKnowledgeBase::new(documents.clone());
        assert_eq!(kb.len(), 2);
        assert_eq!(kb.documents, documents);
    }

    #[test]
    fn test_add_document() {
        let doc1 = create_test_document("1", "Content 1");
        let mut kb = DocumentKnowledgeBase::new(vec![doc1.clone()]);
        assert_eq!(kb.len(), 1);

        let doc2 = create_test_document("2", "Content 2");
        kb.add_document(doc2.clone());
        assert_eq!(kb.len(), 2);
        assert_eq!(kb.documents.get(1), Some(&doc2));
    }

    #[test]
    fn test_is_empty() {
        let kb_empty = DocumentKnowledgeBase::new(Vec::new());
        assert!(kb_empty.is_empty());

        let doc1 = create_test_document("1", "Content 1");
        let kb_not_empty = DocumentKnowledgeBase::new(vec![doc1]);
        assert!(!kb_not_empty.is_empty());
    }

    #[test]
    fn test_iter_documents() {
        let doc1 = create_test_document("1", "Content 1");
        let doc2 = create_test_document("2", "Content 2");
        let documents = vec![doc1.clone(), doc2.clone()];
        let kb = DocumentKnowledgeBase::new(documents.clone());

        let mut iter = kb.iter_documents();
        assert_eq!(iter.next(), Some(&doc1));
        assert_eq!(iter.next(), Some(&doc2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_document_lists_iter() {
        let doc1 = create_test_document("1", "Content 1");
        let doc2 = create_test_document("2", "Content 2");
        let documents_original = vec![doc1.clone(), doc2.clone()];
        let kb = DocumentKnowledgeBase::new(documents_original.clone());

        let mut iter = kb.document_lists_iter();

        let first_list = iter.next();
        assert!(first_list.is_some());
        assert_eq!(first_list.unwrap(), vec![doc1.clone()]);

        let second_list = iter.next();
        assert!(second_list.is_some());
        assert_eq!(second_list.unwrap(), vec![doc2.clone()]);

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_document_lists_iter_empty() {
        let kb = DocumentKnowledgeBase::new(Vec::new());
        let mut iter = kb.document_lists_iter();
        assert_eq!(iter.next(), None);
    }
}
