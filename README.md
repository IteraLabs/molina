# molina
An LLM-based research assistant project.

# tech stack

- Python, Docker
- torch, torchtune
- Llama3
- pdfplumber

# techniques

- Named Entity Recognition (NER): To identify key entities (SpaCy).
- Relation Extraction (RE): To extract relationship between entities (SpaCy).

- Latent Dirichlet Allocation: To identify topics.
- Text extraction: to extract text from PDFs (pdfPlumber).
- Prompt Engineering: Prompt templates (llama-3).

# content
- Academic papers, books
- Metadata

# Blocks

1. Content ETL
    1. Extract from PDF
    2. Transform it with NERRE 
    3. Load the transformed content into a metadata catalaog.

# Install dependencies

```shell
python -m spacy download en_core_web_trf
```

# References

- https://universaldependencies.org/u/pos/

