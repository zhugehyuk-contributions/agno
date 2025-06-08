use pyo3::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json;

/// Represents a document with content and associated metadata.
///
/// This struct is exposed to Python as the `Document` class.
/// It can be serialized to and deserialized from JSON.
#[pyclass(name = "Document")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Document {
    /// The main textual content of the document.
    #[pyo3(get, set)]
    pub content: String,

    /// An optional unique identifier for the document.
    #[pyo3(get, set)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// An optional name or title for the document.
    #[pyo3(get, set)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional metadata associated with the document, stored as a map
    /// where keys are strings and values are flexible JSON types.
    /// Defaults to an empty map.
    #[serde(default)]
    pub meta_data: HashMap<String, serde_json::Value>,

    /// Optional usage information, typically related to processing or embedding.
    /// For example, token counts from an API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<HashMap<String, serde_json::Value>>,

    /// An optional score, often used for search result ranking.
    #[pyo3(get, set)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reranking_score: Option<f64>,
}

#[pymethods]
impl Document {
    /// Creates a new `Document` instance.
    ///
    /// Args:
    ///     content (str): The main textual content of the document.
    ///     id (Optional[str]): An optional unique identifier. Defaults to None.
    ///     name (Optional[str]): An optional name or title. Defaults to None.
    ///     reranking_score (Optional[float]): An optional score. Defaults to None.
    ///
    /// Note:
    ///     `meta_data` and `usage` fields are initialized to their defaults (empty map and None, respectively)
    ///     and can be modified via methods like `set_meta_data_from_json`.
    #[new]
    #[pyo3(signature = (content, id=None, name=None, reranking_score=None))]
    fn py_new(
        content: String,
        id: Option<String>,
        name: Option<String>,
        reranking_score: Option<f64>
    ) -> Self {
        Document {
            content,
            id,
            name,
            meta_data: HashMap::new(),
            usage: None,
            reranking_score,
        }
    }

    /// Serializes the `Document` instance to a JSON string.
    ///
    /// Returns:
    ///     str: The JSON string representation of the document.
    ///
    /// Raises:
    ///     PyValueError: If serialization fails.
    #[pyo3(name = "to_json")]
    fn to_json_py(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    /// Deserializes a `Document` instance from a JSON string.
    ///
    /// Args:
    ///     json_str (str): The JSON string representation of a document.
    ///
    /// Returns:
    ///     Document: A new instance of the Document.
    ///
    /// Raises:
    ///     PyValueError: If deserialization fails.
    #[staticmethod]
    #[pyo3(name = "from_json")]
    fn from_json_py(json_str: &str) -> PyResult<Self> {
        serde_json::from_str(json_str).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    /// Retrieves the `meta_data` field as a JSON string.
    ///
    /// Returns:
    ///     str: A JSON string representation of the `meta_data` map.
    ///
    /// Raises:
    ///     PyValueError: If serialization of `meta_data` fails.
    #[pyo3(name = "get_meta_data_as_json")]
    fn get_meta_data_as_json_py(&self) -> PyResult<String> {
        serde_json::to_string(&self.meta_data).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    /// Sets the `meta_data` field from a JSON string.
    ///
    /// Args:
    ///     json_str (str): A JSON string representing the new meta_data map.
    ///
    /// Raises:
    ///     PyValueError: If deserialization of the JSON string into meta_data fails.
    #[pyo3(name = "set_meta_data_from_json")]
    fn set_meta_data_from_json_py(&mut self, json_str: &str) -> PyResult<()> {
        self.meta_data = serde_json::from_str(json_str).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(())
    }
}

/// Python module for the `document` crate.
///
/// This module exposes the `Document` class, allowing creation and manipulation
/// of document objects from Python, with the underlying implementation in Rust
/// for performance benefits.
#[pymodule]
fn document(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Document>()?;
    Ok(())
}

// --- Original Rust tests ----
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_serialization_deserialization_with_meta() {
        let mut meta = HashMap::new();
        meta.insert("key".to_string(), serde_json::json!("value"));

        let doc = Document {
            content: "Test content".to_string(),
            id: Some("123".to_string()),
            name: Some("Test Document".to_string()),
            meta_data: meta.clone(),
            usage: None,
            reranking_score: Some(0.95),
        };

        let serialized_json = serde_json::to_string(&doc).unwrap();

        assert!(serialized_json.contains("\"content\":\"Test content\""));
        assert!(serialized_json.contains("\"id\":\"123\""));
        assert!(serialized_json.contains("\"name\":\"Test Document\""));
        assert!(serialized_json.contains("\"meta_data\":{\"key\":\"value\"}"));
        assert!(serialized_json.contains("\"reranking_score\":0.95"));
        assert!(!serialized_json.contains("usage"));

        let deserialized_doc: Document = serde_json::from_str(&serialized_json).unwrap();
        assert_eq!(doc, deserialized_doc);
    }

    #[test]
    fn test_document_minimal_serialization_with_default_meta() {
        let doc = Document {
            content: "Minimal content".to_string(),
            id: None,
            name: None,
            meta_data: HashMap::new(),
            usage: None,
            reranking_score: None,
        };
        let serialized_json = serde_json::to_string(&doc).unwrap();
        assert_eq!(serialized_json, r#"{"content":"Minimal content","meta_data":{}}"#);

        let deserialized_doc: Document = serde_json::from_str(&serialized_json).unwrap();
        assert_eq!(doc, deserialized_doc);

        let json_minimal_input = r#"{"content":"Minimal content from JSON"}"#;
        let deserialized_minimal_doc: Document = serde_json::from_str(json_minimal_input).unwrap();
        assert_eq!(deserialized_minimal_doc.content, "Minimal content from JSON");
        assert!(deserialized_minimal_doc.meta_data.is_empty());
        assert_eq!(deserialized_minimal_doc.id, None);
    }

    #[test]
    fn test_document_with_empty_meta_serialization() {
        let doc = Document {
            content: "Content with empty meta".to_string(),
            id: None,
            name: None,
            meta_data: HashMap::new(),
            usage: None,
            reranking_score: None,
        };
        let serialized_json = serde_json::to_string(&doc).unwrap();
        assert!(serialized_json.contains("\"meta_data\":{}"));
        assert!(serialized_json.contains("\"content\":\"Content with empty meta\""));

        let deserialized_doc: Document = serde_json::from_str(&serialized_json).unwrap();
        assert_eq!(doc, deserialized_doc);
    }

    #[test]
    fn test_deserialization_missing_optional_fields_and_default_meta() {
        let json_input = r#"{"content":"Content only"}"#;
        let deserialized_doc: Document = serde_json::from_str(json_input).unwrap();
        let expected_doc = Document {
            content: "Content only".to_string(),
            id: None,
            name: None,
            meta_data: HashMap::new(),
            usage: None,
            reranking_score: None,
        };
        assert_eq!(deserialized_doc, expected_doc);
    }

     #[test]
    fn test_deserialization_with_all_fields_including_meta() {
        let json_input = r#"{
            "content": "Full document",
            "id": "doc_id_001",
            "name": "Full Doc",
            "meta_data": {"source": "test", "valid": true},
            "usage": {"tokens": 120},
            "reranking_score": 0.88
        }"#;
        let deserialized_doc: Document = serde_json::from_str(json_input).unwrap();
        let mut meta = HashMap::new();
        meta.insert("source".to_string(), serde_json::json!("test"));
        meta.insert("valid".to_string(), serde_json::json!(true));
        let mut usage_map = HashMap::new();
        usage_map.insert("tokens".to_string(), serde_json::json!(120));
        let expected_doc = Document {
            content: "Full document".to_string(),
            id: Some("doc_id_001".to_string()),
            name: Some("Full Doc".to_string()),
            meta_data: meta,
            usage: Some(usage_map),
            reranking_score: Some(0.88),
        };
        assert_eq!(deserialized_doc, expected_doc);
    }
}
