import time
import json
import sys
import os
from typing import Optional, Dict, Any, List # For PythonDocument typing

# --- Simple Python Document Class for Comparison ---
class PythonDocument:
    def __init__(self,
                 content: str,
                 id: Optional[str] = None,
                 name: Optional[str] = None,
                 meta_data: Optional[Dict[str, Any]] = None,
                 reranking_score: Optional[float] = None,
                 # Add usage if needed for closer comparison to original Python, though Rust one doesn't use it in constructor
                 usage: Optional[Dict[str, Any]] = None
                ):
        self.content = content
        self.id = id
        self.name = name
        self.meta_data = meta_data if meta_data is not None else {}
        self.reranking_score = reranking_score
        self.usage = usage # Keep for potential deeper comparison

    def to_json(self) -> str:
        # Mimic Rust version's serde behavior: skip Nones, meta_data is always present
        data = {"content": self.content, "meta_data": self.meta_data}
        if self.id is not None:
            data["id"] = self.id
        if self.name is not None:
            data["name"] = self.name
        if self.reranking_score is not None:
            data["reranking_score"] = self.reranking_score
        if self.usage is not None:
            data["usage"] = self.usage
        return json.dumps(data)

    @classmethod
    def from_json(cls, json_str: str) -> 'PythonDocument':
        data = json.loads(json_str)
        return cls(**data)

# --- Rust Document FFI Import ---
try:
    import document as rust_document
except ImportError as e:
    print(f"Failed to import 'document' (Rust FFI module). Ensure it's compiled and in PYTHONPATH.", file=sys.stderr)
    sys.path.append("/app/rust_project/target/release") # Fallback for this script
    try:
        import document as rust_document
        print("Successfully imported 'document' module from fallback path.", file=sys.stderr)
    except ImportError:
        print(f"Fallback import also failed. PYTHONPATH: {os.environ.get('PYTHONPATH')}, sys.path: {sys.path}", file=sys.stderr)
        raise e

# --- Benchmark Parameters ---
N_ITERATIONS = 10000  # Number of iterations for each operation

sample_data_for_creation = {
    "content": "This is benchmark content.",
    "id": "doc_benchmark_123",
    "name": "Benchmark Document",
    "reranking_score": 0.95
}

sample_json_string = json.dumps({
    "content": "Sample content for deserialization.",
    "id": "doc_deser_456",
    "name": "Deserialized Benchmark Doc",
    "meta_data": {"source": "benchmark", "status": "active"},
    "reranking_score": 0.88,
    "usage": {"tokens": 50}
})

def benchmark_python_document():
    print(f"\nBenchmarking PythonDocument ({N_ITERATIONS} iterations)...")

    # 1. Object Creation
    start_time = time.perf_counter()
    for _ in range(N_ITERATIONS):
        doc = PythonDocument(
            content=sample_data_for_creation["content"],
            id=sample_data_for_creation["id"],
            name=sample_data_for_creation["name"],
            reranking_score=sample_data_for_creation["reranking_score"]
            # meta_data and usage will be default
        )
    end_time = time.perf_counter()
    creation_time = end_time - start_time
    print(f"  Python Creation:      {creation_time:.6f} seconds")

    # Create one instance for ser/deser tests
    doc_instance = PythonDocument(**json.loads(sample_json_string))

    # 2. Serialization to JSON
    start_time = time.perf_counter()
    for _ in range(N_ITERATIONS):
        doc_instance.to_json()
    end_time = time.perf_counter()
    serialization_time = end_time - start_time
    print(f"  Python Serialization:   {serialization_time:.6f} seconds")

    # 3. Deserialization from JSON
    json_str_for_deser = doc_instance.to_json() # Use a consistent JSON string
    start_time = time.perf_counter()
    for _ in range(N_ITERATIONS):
        PythonDocument.from_json(json_str_for_deser)
    end_time = time.perf_counter()
    deserialization_time = end_time - start_time
    print(f"  Python Deserialization: {deserialization_time:.6f} seconds")

    return creation_time, serialization_time, deserialization_time

def benchmark_rust_document():
    print(f"\nBenchmarking Rust FFI Document ({N_ITERATIONS} iterations)...")

    # 1. Object Creation
    start_time = time.perf_counter()
    for _ in range(N_ITERATIONS):
        doc = rust_document.Document(
            content=sample_data_for_creation["content"],
            id=sample_data_for_creation["id"],
            name=sample_data_for_creation["name"],
            reranking_score=sample_data_for_creation["reranking_score"]
            # meta_data and usage will be default in Rust constructor
        )
    end_time = time.perf_counter()
    creation_time = end_time - start_time
    print(f"  Rust Creation:      {creation_time:.6f} seconds")

    # Create one instance for ser/deser tests
    doc_instance = rust_document.Document.from_json(sample_json_string)

    # 2. Serialization to JSON (using to_json_py method)
    start_time = time.perf_counter()
    for _ in range(N_ITERATIONS):
        doc_instance.to_json() # This calls to_json_py
    end_time = time.perf_counter()
    serialization_time = end_time - start_time
    print(f"  Rust Serialization:   {serialization_time:.6f} seconds")

    # 3. Deserialization from JSON (using from_json_py method)
    json_str_for_deser = doc_instance.to_json() # Use a consistent JSON string
    start_time = time.perf_counter()
    for _ in range(N_ITERATIONS):
        rust_document.Document.from_json(json_str_for_deser)
    end_time = time.perf_counter()
    deserialization_time = end_time - start_time
    print(f"  Rust Deserialization: {deserialization_time:.6f} seconds")

    return creation_time, serialization_time, deserialization_time

if __name__ == "__main__":
    print("Starting benchmarks...")

    py_creation, py_ser, py_deser = benchmark_python_document()
    rs_creation, rs_ser, rs_deser = benchmark_rust_document()

    print("\n--- Summary ---")
    print(f"Iterations per test: {N_ITERATIONS}")
    print("\nCreation Time:")
    print(f"  Python: {py_creation:.6f} s")
    print(f"  Rust:   {rs_creation:.6f} s")
    if py_creation > 0 and rs_creation > 0:
        print(f"  Factor (Py/Rust): {py_creation/rs_creation:.2f}x faster for Rust")

    print("\nSerialization Time (to JSON string):")
    print(f"  Python: {py_ser:.6f} s")
    print(f"  Rust:   {rs_ser:.6f} s")
    if py_ser > 0 and rs_ser > 0:
        print(f"  Factor (Py/Rust): {py_ser/rs_ser:.2f}x faster for Rust")

    print("\nDeserialization Time (from JSON string):")
    print(f"  Python: {py_deser:.6f} s")
    print(f"  Rust:   {rs_deser:.6f} s")
    if py_deser > 0 and rs_deser > 0:
        print(f"  Factor (Py/Rust): {py_deser/rs_deser:.2f}x faster for Rust")

    # Note: Actual performance can vary greatly based on system load,
    # Python version, Rust optimization levels, and the complexity of the data.
    # This is a very simple micro-benchmark.
    print("\nBenchmark finished.")
