# RAG system blocks
---

## Stages

- Knowledge-base
    - Local Source System
    - Content ETL+T

- Pre-retrieval
    - Query Manipulation
    - Data Modification

- Retrieval Step
    - Search & Rank
    - Strategy

- Post-retrieval
    - Re-ranking
    - Filtering

- Generation
    - Enhancing
    - Customization

### Knoledge-base

**Local Source System**

- **Folder Structure**: The `knowledge/` with 4 venues.
- **Agnostic Source Catalog File**: `source_catalog.json`.
- **Content has been downloaded**: yes (72 articles from 3 different venues).

**Content ETL+T**

- Extract:
    - v0.1.0 : filter fields, read a PDF file, split in pages, python compatible.
    - (Rust) define a meta-data filter to read pdf.
    - (Rust) read 1 pdf file (using the previously defined filter).
    - (Rust) split the content page by page.
    - (Rust) pass the result in a python-compatible dictionary.

- Transform:
    - v0.1.0 : Select only text, only python-compatible dict format, a single implementation of a tokenizer.
    - Part 1 : Remove and Regroup
        - (Rust) drop anything that is not text.
        - (Rust) re-group into common sections (abstract, introduction, etc.).
        - (Rust) define filter to apply into text.
        - (Rust) filter the grouped text.
        - (Rust) pass the result in a python-compatible dictionary.
    - Part 2 : Tokenize
        - (Python) tokenize section-wise.

- Load:
    - v0.1.0 : Not yet defined.
    - (Shell - Rust) Create a vectror DB with chroma.
    - (Rust) inject into vector DB.

- Test:
    - v0.1.0 : Not yet defined.
    - (Rust) corresponding functionality and integration tests.
    - (Python) corresponding functionality and integreation tests. 

