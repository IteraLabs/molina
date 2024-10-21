
import os
import shutil 
import warnings
warnings.filterwarnings("ignore", category=UserWarning)

from langchain_core.documents import Document
# from langchain_community.vectorstores.chroma import Chroma 
from langchain_community.embeddings import GPT4AllEmbeddings
from content import documents
from generators import gpt4all
# last
from langchain_chroma import Chroma

# -- ------------------------------------------------------------------------------- -- #
# -- ------------------------------------------------------------------------------- -- #

def create_vectordb(db_path: str, chunks: list[Document], embedding_model, verbose):
    """
    Save the given list of Document objects to a Chroma database.
  
    Args:
        chunks (list[Document]): List of Document objects representing text chunks to save.
    
    Returns:
        None
    """

    # Clear out the existing database directory if it exists
    if os.path.exists(db_path):
        shutil.rmtree(db_path)

    gguf_model_name = "all-MiniLM-L6-v2.gguf2.f16.gguf"
    gpt4all_kwargs = {'allow_download': 'True'}

    if embedding_model is None:
        embedding_function = GPT4AllEmbeddings(model_name=gguf_model_name, 
                                               gpt4all_kwargs=gpt4all_kwargs)
    else:
        embedding_function = embedding_model

    # Create a new Chroma database from the documents using the 
    # Provided embeddings

    db3 = Chroma.from_documents(
            documents = chunks,
            collection_name = "local-pdf-chroma",
            embedding=embedding_function,
            persist_directory=db_path,)

    # Print actions
    if verbose:
        print(f"Saved {len(chunks)} chunks to {db_path}.")

    return db3 

# -- ------------------------------------------------------------------------------- -- #
# -- ------------------------------------------------------------------------------- -- #

def search_vectordb(client, query):
     
    found_search = client.similarity_search(query)
    found_l = []
    
    for i in range(0, len(found_search)):
        found_l.append(found_search[i].model_dump()["page_content"])

    return found_l

# -- ------------------------------------------------------------------------------- -- #
# -- ------------------------------------------------------------------------------- -- #

def setup_vectordb(db_path: str, docs_path: str):
    """
    Setup the vector database with documents.
    """
    
    k_documents = documents.load_documents(data_path=docs_path)
    ks_documents = documents.split_text(k_documents, c_size=1000, c_overlap=200)

    chromadb = create_vectordb(
        db_path=db_path,
        chunks=ks_documents,
        embedding_model=gpt4all.embedding_model,
        verbose=True
    )

    return chromadb

