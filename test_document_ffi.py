import unittest
import sys
import os
import json

# Add the directory containing document.so to Python's search path
try:
    import document
except ImportError as e:
    print(f"Failed to import 'document' module. Ensure document.so is in PYTHONPATH.", file=sys.stderr)
    print(f"PYTHONPATH: {os.environ.get('PYTHONPATH')}", file=sys.stderr)
    print(f"sys.path: {sys.path}", file=sys.stderr)
    sys.path.append("/app/rust_project/target/release")
    try:
        import document
    except ImportError:
        print("Fallback import also failed.", file=sys.stderr)
        raise e


class TestDocumentFFI(unittest.TestCase):

    def test_01_create_document_and_access_fields(self):
        print("Running test_01_create_document_and_access_fields")
        doc = document.Document("Initial content", id="doc_01", name="Test Name", reranking_score=0.85)
        self.assertEqual(doc.content, "Initial content")
        self.assertEqual(doc.id, "doc_01")
        self.assertEqual(doc.name, "Test Name")
        self.assertEqual(doc.reranking_score, 0.85)

        doc.content = "Updated content"
        self.assertEqual(doc.content, "Updated content")
        doc.id = "doc_01_updated"
        self.assertEqual(doc.id, "doc_01_updated")

    def test_02_to_json(self):
        print("Running test_02_to_json")
        doc_data = {
            "content": "JSON test content",
            "id": "json_01",
            "name": "JSON Doc",
            "reranking_score": 0.99
        }
        doc = document.Document(
            doc_data["content"],
            id=doc_data["id"],
            name=doc_data["name"],
            reranking_score=doc_data["reranking_score"]
        )

        json_str = doc.to_json()
        py_dict = json.loads(json_str)

        self.assertEqual(py_dict["content"], doc_data["content"])
        self.assertEqual(py_dict["id"], doc_data["id"])
        self.assertEqual(py_dict["name"], doc_data["name"])
        self.assertEqual(py_dict["reranking_score"], doc_data["reranking_score"])
        self.assertEqual(py_dict["meta_data"], {})
        self.assertNotIn("usage", py_dict)

    def test_03_from_json(self):
        print("Running test_03_from_json")
        # Using a standard triple-quoted string for clarity and to avoid indentation issues within the string
        json_input = """{
            "content": "From JSON string",
            "id": "from_json_01",
            "name": "From JSON Name",
            "meta_data": {"source": "json_source"},
            "reranking_score": 0.77
        }"""

        doc = document.Document.from_json(json_input)
        self.assertEqual(doc.content, "From JSON string")
        self.assertEqual(doc.id, "from_json_01")
        self.assertEqual(doc.name, "From JSON Name")
        self.assertEqual(doc.reranking_score, 0.77)

        meta_json = doc.get_meta_data_as_json()
        meta_dict = json.loads(meta_json)
        self.assertEqual(meta_dict, {"source": "json_source"})

    def test_04_meta_data_json_methods(self):
        print("Running test_04_meta_data_json_methods")
        doc = document.Document("Meta test")

        initial_meta_json = doc.get_meta_data_as_json()
        self.assertEqual(json.loads(initial_meta_json), {})

        new_meta_data = {"key1": "value1", "key2": 42}
        doc.set_meta_data_from_json(json.dumps(new_meta_data))

        retrieved_meta_json = doc.get_meta_data_as_json()
        self.assertEqual(json.loads(retrieved_meta_json), new_meta_data)

    def test_05_optional_fields_none(self):
        print("Running test_05_optional_fields_none")
        doc = document.Document("Only content here")
        self.assertEqual(doc.content, "Only content here")
        self.assertIsNone(doc.id)
        self.assertIsNone(doc.name)
        self.assertIsNone(doc.reranking_score)

        json_str = doc.to_json()
        py_dict = json.loads(json_str)
        self.assertEqual(py_dict["content"], "Only content here")
        self.assertEqual(py_dict["meta_data"], {})
        self.assertNotIn("id", py_dict)
        self.assertNotIn("name", py_dict)
        self.assertNotIn("reranking_score", py_dict)

if __name__ == "__main__":
    unittest.main()
