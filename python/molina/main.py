import warnings
warnings.filterwarnings("ignore", category=UserWarning)

from content import documents, vector
from generators import llama, gpt4all

def main():

    """
    """
    
    # -- -------------------------------------------- Extract/Load/Transform Content -- #
    # -- --------------------------------------------------------------------------- -- #
    
    db_path = "knowledge/chroma"
    docs_path = "knowledge/molina_compatible"
    
    wd_folder = "/Users/franciscome/git/iteralabs/molina"
    in_knowledge = "/knowledge"
    in_topic = "/molina_compatible"
    in_folder = wd_folder + in_knowledge + in_topic

    # -- -------------------------------------------- Extract/Load/Transform Content -- #
    # -- --------------------------------------------------------------------------- -- #
   
    k_documents = documents.load_documents(data_path=docs_path)
    ks_documents = documents.split_text(k_documents, c_size=1000, c_overlap=100)

    # -- ---------------------------------------------------------- Create Documents -- #
    # -- ---------------------------------------------------------- ---------------- -- #

    chromadb = vector.create_vectordb(db_path=db_path,
                           chunks=ks_documents, 
                           embedding_model=gpt4all.embedding_model,
                           verbose=True)

    # -- ------------------------------------------------------ Create Embeddings DB -- #
    # -- --------------------------------------------------------------------------- -- #

    context_query = "large language models, llm"
    
    generation_query = "what is the perplexity of a Retrieval-based language model?"
    
    retrieved_texts = vector.search_vectordb(chromadb, context_query)   
    llama_answer = llama.ask_query(retrieved_texts, generation_query)
    a_specific = llama_answer['response'][-1]['generated_text'][-1]['content']
    
    # -- --------------------------------------------------------- Output Formatting -- #
    # -- --------------------------------------------------------------------------- -- #
    
    print(f'\n agent answer: ', a_specific, '\n')

if __name__ == "__main__":
    main()

