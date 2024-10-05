# RAG System Blocks
---

- Pre-retrieval
	- Indexing
- Retrieval Step
    - Search & Rank
    - Strategy
- Post-retrieval
- Generation


# Pre-Retrieval
---

## Indexing 

Establishes an organized data system to enable fast and accurate retrieval of information. Dense vectors, also known as embeddings, capture the semantic meaning of words and documents, allowing for more flexible and accurate retrieval.

### Granularity

- **Sentence-level** : 
	- For question-answering systems to precisely locate answers.
	- Suitable when users need precise definitions or statements.
- Paragraph-level
	- A balanced choice that captures context while allowing for specific information retrieval.
	- For general inquiries about methodologies or discussions.
- Section-Level
	- Aligns with common academic paper structures (e.g., introduction, methods).
	- Useful for users looking for comprehensive insights into specific parts of a paper.
- **Document-level** 
	- More appropriate for summarizing documents to understand their main concepts and ideas.
- Chunk-Level
	- Dynamic Specificity for particular and well defined implementations.

### Implementation Steps

- Extraction:
	- Extract text from PDFs
- Pre-processing: 
	- Clean and pre-process/format the extracted text.
- Granular division: 
	- Divide the cleaned text into the chosen granularity (e.g. using regex or predefined markers).
- Indexing:
	- Store each chunk in Chroma’s vector store with appropriate metadata (e.g., source document, section title).
- Test:
	- Performance and efficacy metrics (usefulness as defined by the application).


# Retrieval
--- 

- Search & Rank
	- To improve the relevance of retrieved documents.
	- To optimize the retriever’s ability to select contextually relevant documents.
		- Integrates retrieval with chain-of-thought (CoT) reasoning, interleaving these processes to ensure that each retrieval step supports the ongoing reasoning task.
		- Confidence based mechanism to dynamically trigger retrieval when the model generates low-confidence tokens.
- Strategy
	- Linear: 
		- Execution of a single pass of sequential actions.
		- For cases that benefits from simplicity and efficiency.
		- The Atlas \[94], REPLUG \[122]
	- Iterative: 
		- Information is retrieved in multiple steps, each informed by previous results.
		- Effective in scenarios requiring multi-step problem-solving, detailed exploration and refinement. 
		- IRCOT \[131], ITER-RETGEN \[121], RQ-RAG \[12], PlanRAG \[81]
	- Recursive:
		- Retrieval that can call itself, creating a hierarchy or tree of retrievals.
		- Useful for hierarchical data exploration, knowledge graphs construction.
		- SURGE \[70], MEMWALKER \[13], IMRAG \[150], Selfmem \[21]
	- Conditional:
		- Governed by specific rules which may be predefined or dynamically determined during the process.
		- useful for compliance checking, rule-based recommendation systems, and context-sensitive information retrieval.
		- PRCA \[151] RARG \[159], CRAG \[149], 
	- Adaptive:
		- adjusts the retrieval strategy based on the context and nature of the query or the data retrieved so far.
		- personalized search engines, adaptive learning systems, and real-time decision support.
		- AAR \[157], FLARE \[65], SelfRAG \[4], CoK \[86], DRAGIN \[124]



