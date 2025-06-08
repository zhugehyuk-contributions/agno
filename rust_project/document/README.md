# Rust Document Module (FFI)

This document describes how to build the Rust-based `document` module and use its `Document` class from Python. The underlying implementation is in Rust for performance, exposed to Python via PyO3.

## 1. Building the Rust Module

The `document` crate is part of a larger Rust workspace located in the `rust_project` directory.

**Prerequisites:**
- Rust programming language and Cargo (Rust's package manager). Install from [rustup.rs](https://rustup.rs/).
- Python 3 development environment.
- A C compiler (usually available on most systems; needed by PyO3).

**Build Steps:**

1.  **Navigate to the workspace directory:**
    ```bash
    cd path/to/your/rust_project
    ```

2.  **Build the `document` package in release mode:**
    This command compiles the `document` crate specifically. The `--release` flag ensures optimizations for performance.
    ```bash
    cargo build --package document --release
    ```

3.  **Locate the compiled library:**
    The build process will create a shared library file. Its exact name and location can vary slightly by operating system:
    -   **Linux:** `rust_project/target/release/libdocument.so`
    -   **macOS:** `rust_project/target/release/libdocument.dylib`
    -   **Windows:** `rust_project/target/release/document.dll` (Note: on Windows, it might be `document.dll` directly without the `lib` prefix for `cdylib`s).

    The `cargo build --package document --release --message-format=json` command can be used to get the precise path from the JSON output (look for a "compiler-artifact" message with `target.name == "document"` and `crate_types` including `"cdylib"`).

## 2. Using the `Document` Class from Python

To use the Rust-compiled `Document` class in Python, the shared library file must be accessible to Python's import system.

**Setup for Python:**

1.  **Copy and Rename the Library:**
    Copy the compiled library (e.g., `libdocument.so`) to a directory that Python will check for modules (e.g., your project's main directory or a specific `site-packages` directory). Rename it to match the Python module name defined in Rust (which is `document` in this case).
    -   On Linux/macOS: Copy `libdocument.so` (or `.dylib`) to `document.so`.
    -   On Windows: Copy `document.dll` to `document.pyd`.

    Example (from within `rust_project` directory, assuming Linux):
    ```bash
    cp target/release/libdocument.so document.so
    # Or copy it to a specific directory in your PYTHONPATH
    # mkdir -p my_python_project/libs
    # cp target/release/libdocument.so my_python_project/libs/document.so
    ```

2.  **Ensure Python Can Find the Module:**
    -   If you copied `document.so` (or `.pyd`) to the same directory as your Python script, it should usually be importable directly.
    -   Alternatively, add the directory containing `document.so` to your `PYTHONPATH` environment variable.
        ```bash
        export PYTHONPATH="/path/to/directory_containing_document_so:$PYTHONPATH"
        ```
    -   Or, in Python, you can modify `sys.path`:
        ```python
        import sys
        sys.path.append("/path/to/directory_containing_document_so")
        ```

**Example Python Code (`example.py`):**

```python
import document # Imports the Rust FFI module (document.so or document.pyd)
import json

def main():
    # 1. Create a new Document object
    print("--- Creating Document ---")
    doc1 = document.Document(
        content="This is content from Rust FFI!",
        id="rust_doc_001",
        name="My Rust Document",
        reranking_score=0.95
    )
    print(f"Doc1 Content: {doc1.content}")
    print(f"Doc1 ID: {doc1.id}")
    print(f"Doc1 Name: {doc1.name}")
    print(f"Doc1 Reranking Score: {doc1.reranking_score}")

    # 2. Modify fields
    print("\n--- Modifying Document ---")
    doc1.content = "Updated content for doc1."
    doc1.id = "rust_doc_001_v2"
    print(f"Doc1 New Content: {doc1.content}")
    print(f"Doc1 New ID: {doc1.id}")

    # 3. Serialize to JSON
    print("\n--- Serializing to JSON ---")
    json_string = doc1.to_json()
    print(f"Serialized JSON: {json_string}")

    # Verify JSON content
    data = json.loads(json_string)
    assert data["content"] == "Updated content for doc1."
    assert data["id"] == "rust_doc_001_v2"
    assert data["meta_data"] == {} # Default empty meta_data

    # 4. Deserialize from JSON
    print("\n--- Deserializing from JSON ---")
    json_input = r'''{
        "content": "Loaded from JSON",
        "id": "json_doc_002",
        "name": "JSON Loaded Doc",
        "meta_data": {"source": "external_json", "version": 1.0},
        "reranking_score": 0.88
    }'''
    doc2 = document.Document.from_json(json_input)
    print(f"Doc2 Content: {doc2.content}")
    print(f"Doc2 ID: {doc2.id}")
    print(f"Doc2 Name: {doc2.name}")
    print(f"Doc2 Reranking Score: {doc2.reranking_score}")

    # 5. Interact with meta_data via JSON methods
    print("\n--- Interacting with MetaData ---")
    print(f"Doc2 Initial MetaData (JSON): {doc2.get_meta_data_as_json()}")

    new_meta = {"author": "Rust FFI Test", "status": "testing"}
    doc2.set_meta_data_from_json(json.dumps(new_meta))

    retrieved_meta_str = doc2.get_meta_data_as_json()
    print(f"Doc2 Updated MetaData (JSON): {retrieved_meta_str}")
    retrieved_meta = json.loads(retrieved_meta_str)
    assert retrieved_meta["author"] == "Rust FFI Test"

    print("\n--- Example Finished ---")

if __name__ == "__main__":
    main()
```

This `README.md` provides a basic guide for developers to build and use the Rust-based `document` module from Python.
