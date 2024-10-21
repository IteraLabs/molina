
import warnings
warnings.filterwarnings("ignore", category=UserWarning)

from transformers import AutoModelForCausalLM, AutoTokenizer, pipeline
from torch import cuda, bfloat16

# -- ------------------------------------------------------------------------------- -- #
# -- ------------------------------------------------------------------------------- -- #

def embeddings_model():
    """

    # to access to the model, for the case of meta-llama
    # You need to agree to share your contact information

    # For my user the following has already been granted
    # https://huggingface.co/meta-llama/Llama-3.2-3B

    # consequently, a login is needed 

    """
    
    checkpoint = "meta-llama/Llama-3.2-3B-Instruct"
    generator = AutoTokenizer.from_pretrained(checkpoint)
    
    return generator

# -- ------------------------------------------------------------------------------- -- #
# -- ------------------------------------------------------------------------------- -- #

def generation_model():
    """

    """

    checkpoint = "meta-llama/Llama-3.2-3B-Instruct"
    llama_tokenizer = embeddings_model()

    compute_device = f'cuda:{cuda.current_device()}' if cuda.is_available() else 'mps'

    llama_model = AutoModelForCausalLM.from_pretrained(checkpoint,
                                                       quantization_config=None,
                                                       torch_dtype=bfloat16)
    generator = pipeline("text-generation",
                         model=llama_model, 
                         tokenizer=llama_tokenizer, 
                         device=compute_device)
    return generator

# ---------------------------------------------------------------------------------- -- #
# ---------------------------------------------------------------------------------- -- #

def ask_query(retrieved_texts, generation_query, generation_model):
    """

    """

    # Prepare the messages for the text generation pipeline
    messages = [
        
        {"role": "system", "content": "You are a helpful AI assistant."
                 "Provide one Answer ONLY the following query based on the context provided below. "
                 "Do not generate or answer any other questions. "
                 "Do not make up or infer any information that is not directly stated in the context. "
                 "Provide a concise answer. "
                 
                f"{retrieved_texts}"
         },

        {"role": "user",
         "content": generation_query}
    
    ]

    # Generate a response using the text generation pipeline
    # generator = generation_model()
    response = generation_model(messages, max_new_tokens=128)
    r_answer = {'query': generation_query, 'context': retrieved_texts, 'response': response}
    
    return r_answer

