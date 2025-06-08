use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MemoryRetrieval {
    LastN,
    FirstN,
    Semantic,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Memory {
    pub memory: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
}

// The Pydantic `to_dict()` with `exclude_none=True` functionality
// is achieved by `#[serde(skip_serializing_if = "Option::is_none")]` on Option fields
// and serde's default behavior for serializing structs to map-like structures (e.g., JSON objects).

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_memory_retrieval_serialization() {
        assert_eq!(serde_json::to_string(&MemoryRetrieval::LastN).unwrap(), r#""last_n""#);
        assert_eq!(serde_json::to_string(&MemoryRetrieval::FirstN).unwrap(), r#""first_n""#);
        assert_eq!(serde_json::to_string(&MemoryRetrieval::Semantic).unwrap(), r#""semantic""#);
    }

    #[test]
    fn test_memory_retrieval_deserialization() {
        assert_eq!(serde_json::from_str::<MemoryRetrieval>(r#""last_n""#).unwrap(), MemoryRetrieval::LastN);
        assert_eq!(serde_json::from_str::<MemoryRetrieval>(r#""first_n""#).unwrap(), MemoryRetrieval::FirstN);
        assert_eq!(serde_json::from_str::<MemoryRetrieval>(r#""semantic""#).unwrap(), MemoryRetrieval::Semantic);
    }

    #[test]
    fn test_memory_serialization_all_fields() {
        let mem = Memory {
            memory: "Recall this event.".to_string(),
            id: Some("mem_123".to_string()),
            topic: Some("event_recollection".to_string()),
            input: Some("User query about event X".to_string()),
        };
        let json = serde_json::to_string(&mem).unwrap();
        // Order of fields in JSON is not guaranteed, so check for presence
        assert!(json.contains(r#""memory":"Recall this event.""#));
        assert!(json.contains(r#""id":"mem_123""#));
        assert!(json.contains(r#""topic":"event_recollection""#));
        assert!(json.contains(r#""input":"User query about event X""#));
    }

    #[test]
    fn test_memory_serialization_mandatory_only() {
        let mem = Memory {
            memory: "Recall this basic fact.".to_string(),
            id: None,
            topic: None,
            input: None,
        };
        let json = serde_json::to_string(&mem).unwrap();
        // Only 'memory' should be present, others are None and skipped
        assert_eq!(json, r#"{"memory":"Recall this basic fact."}"#);
    }

    #[test]
    fn test_memory_deserialization_all_fields() {
        let json_data = r#"{
            "memory": "Recall this event.",
            "id": "mem_123",
            "topic": "event_recollection",
            "input": "User query about event X"
        }"#;
        let deserialized_mem: Memory = serde_json::from_str(json_data).unwrap();

        assert_eq!(deserialized_mem.memory, "Recall this event.");
        assert_eq!(deserialized_mem.id, Some("mem_123".to_string()));
        assert_eq!(deserialized_mem.topic, Some("event_recollection".to_string()));
        assert_eq!(deserialized_mem.input, Some("User query about event X".to_string()));
    }

    #[test]
    fn test_memory_deserialization_mandatory_only() {
        let json_data = r#"{"memory": "Recall this basic fact."}"#;
        let deserialized_mem: Memory = serde_json::from_str(json_data).unwrap();

        assert_eq!(deserialized_mem.memory, "Recall this basic fact.");
        assert_eq!(deserialized_mem.id, None);
        assert_eq!(deserialized_mem.topic, None);
        assert_eq!(deserialized_mem.input, None);
    }

    #[test]
    fn test_memory_deserialization_with_some_optional() {
        let json_data = r#"{
            "memory": "Another memory item.",
            "topic": "general_knowledge"
        }"#;
        let deserialized_mem: Memory = serde_json::from_str(json_data).unwrap();

        assert_eq!(deserialized_mem.memory, "Another memory item.");
        assert_eq!(deserialized_mem.id, None);
        assert_eq!(deserialized_mem.topic, Some("general_knowledge".to_string()));
        assert_eq!(deserialized_mem.input, None);
    }
}
