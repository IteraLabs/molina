import warnings
warnings.filterwarnings("ignore", category=UserWarning)

from content import vector
from generators import llama

def chat_with_user(chromadb, model_gen):
    """
    Chat interface for user interaction.
    """

    print("Welcome to the Chatbot! Type 'exit' to quit.")
    
    while True:
        context_query = input("\nEnter context query: ")
        if context_query.lower() == 'exit':
            break
        
        generation_query = input("Enter your question: ")
        if generation_query.lower() == 'exit':
            break
        
        # Retrieve relevant texts based on the context query
        retrieved_texts = vector.search_vectordb(chromadb, context_query)
        
        # Generate an answer based on the retrieved texts
        llama_answer = llama.ask_query(retrieved_texts, generation_query, model_gen)
        a_specific = llama_answer['response'][-1]['generated_text'][-1]['content']
        
        # Output the results
        print(f'\nProvided Context:\n{retrieved_texts}\n')
        print(f'Agent Answer:\n{a_specific}\n')

def main():

    """
    """
    
    # -- -------------------------------------------- Extract/Load/Transform Content -- #
    # -- --------------------------------------------------------------------------- -- #
    
    db_path = "knowledge/chroma"
    docs_path = "knowledge/molina_compatible"
    
    # -- -------------------------------------------- Extract/Load/Transform Content -- #
    # -- --------------------------------------------------------------------------- -- #
   
    chromadb = vector.setup_vectordb(db_path=db_path, docs_path=docs_path)

    # -- ------------------------------------------------------ Create Embeddings DB -- #
    # -- --------------------------------------------------------------------------- -- #
   
    model_gen = llama.generation_model()

    """

    context: augmented retrieval generation RAG retrieval-based LLM Language Models  
    question: what are the major drawbacks of Retrival augmented generation RAG models ?

    """

    # Start chat with user
    chat_with_user(chromadb, model_gen)

if __name__ == "__main__":
    main()

