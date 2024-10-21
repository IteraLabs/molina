import warnings
warnings.filterwarnings("ignore", category=UserWarning)

from langchain_community.document_loaders.pdf import PyPDFDirectoryLoader 
from langchain_text_splitters import RecursiveCharacterTextSplitter
from langchain_core.documents import Document

# -- ------------------------------------------------------------------------------- -- #
# -- ------------------------------------------------------------------------------- -- #

def create_document(in_text:str, in_metadata: dict):
    """
    From text and metadata, create a Document as usable by Langchain

    Args:
        content: str
            The actual content to be held by the Document.

        metadata: str
            Any metadata useful to describe or inform something about the Document

    Returns:
        Document object as compatible by Langchain

    """

    doc = Document(
        page_content=in_text,
        metadata=in_metadata,)

    return doc

# -- ------------------------------------------------------------------------------- -- #
# -- ------------------------------------------------------------------------------- -- #

def load_documents(data_path):
    """
    Load PDF documents from the specified directory using PyPDFDirectoryLoader.

    Returns:
        List of Document objects: 
        Loaded PDF documents represented as Langchain Document objects.

    Ref:
    [1] https://python.langchain.com/v0.2/docs/concepts/#document-loaders
    [2] https://python.langchain.com/v0.2/docs/how_to/document_loader_pdf/

    """

    # Initialize PDF loader with specified directory
    document_loader = PyPDFDirectoryLoader(data_path) 
    
    # Load PDF documents and return them as a list of Document objects
    return document_loader.load() 

# -- ------------------------------------------------------------------------------- -- #
# -- ------------------------------------------------------------------------------- -- #

def split_text(documents: list[Document], c_size, c_overlap):
    """
    Split the text content of the given list of Document objects into smaller chunks.

    Args:
    documents (list[Document]): List of Document objects containing text content to split.

    Returns:
    list[Document]: List of Document objects representing the split text chunks.

    """

    # Initialize text splitter with specified parameters
    text_splitter = RecursiveCharacterTextSplitter(
        chunk_size=c_size,       # Size of each chunk in characters. 500
        chunk_overlap=c_overlap, # Overlap between consecutive chunks. 100
        length_function=len,     # Function to compute the length of the text
        add_start_index=True,    # Flag to add start index to each chunk
    )

    # Split documents into smaller chunks using text splitter
    chunks = text_splitter.split_documents(documents)
    
    # print(f"Split {len(documents)} documents into {len(chunks)} chunks.")
    
    # Print example of page content and metadata for a chunk
    
    # document = chunks[0]
    # print(document.page_content)
    # print(document.metadata)

    return chunks # Return the list of split text chunks

