
from molina import extract_content
import torch
from torch import cuda
import transformers
from transformers import AutoTokenizer
from time import time
from huggingface_hub import login
login(token="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")

def main():

    """
    !pip install transformers==4.33.0 accelerate==0.22.0 einops==0.6.1 \
    langchain==0.0.300 xformers==0.0.21 bitsandbytes==0.41.1 \
    sentence_transformers==2.2.2 chromadb==0.4.12
    """

    wd_folder = "/Users/franciscome/git/iteralabs/molina"
    in_folder = "/knowledge"
    in_subfolder = "/conference_icml"
    in_file = "/basu24a.pdf"
    in_pdf = wd_folder + in_folder + in_subfolder + in_file
    # in_tokenizer = wd_folder + "/models/Meta-Llama-3-8B-Instruct/"

    # -- --------------------------------------------------------------------------- -- #
    # -- --------------------------------------------------------------------------- -- #
    
    result_content = extract_content(input_file=in_pdf)

    # print(f"the resulting keys were: {result_content.keys()}")
    # print(result_content[1])

    # -- --------------------------------------------------------------------------- -- #
    # -- --------------------------------------------------------------------------- -- #

    model_id = 'meta-llama/Llama-3.2-3B-Instruct'
    tokenizer = AutoTokenizer.from_pretrained(model_id)

    device = f'cuda:{cuda.current_device()}' if cuda.is_available() else 'mps'
    print(device)
    
    time_start = time()
    model_config = transformers.AutoConfig.from_pretrained(
        model_id,
        trust_remote_code=True,
        max_new_tokens=1024
    )

    model = transformers.AutoModelForCausalLM.from_pretrained(
        model_id,
        trust_remote_code=True,
        config=model_config,
        quantization_config=None,
        device_map='auto',
    )

    time_end = time()

    print(f"Prepare model, tokenizer: {round(time_end-time_start, 3)} sec.")

    # -- --------------------------------------------------------------------------- -- #
    # -- --------------------------------------------------------------------------- -- #

    time_start = time()
    query_pipeline = transformers.pipeline(
            "text-generation",
            model=model,
            tokenizer=tokenizer,
            torch_dtype=torch.float16,
            max_length=1024,
            device_map="auto",)
    time_end = time()
    
    print(f"Prepare pipeline: {round(time_end-time_start, 3)} sec.")

    # -- --------------------------------------------------------------------------- -- #
    # -- --------------------------------------------------------------------------- -- #

    def test_model(tokenizer, pipeline, message):
        """
        Perform a query
        print the result
        Args:
            tokenizer: the tokenizer
            pipeline: the pipeline
            message: the prompt
        Returns
            None
        """

        time_start = time()
        sequences = pipeline(
            message,
            do_sample=True,
            top_k=10,
            num_return_sequences=1,
            eos_token_id=tokenizer.eos_token_id,
            max_length=200,)

        time_end = time()
        total_time = f"{round(time_end-time_start, 3)} sec."
        
        question = sequences[0]['generated_text'][:len(message)]
        answer = sequences[0]['generated_text'][len(message):]
        
        return f"Question: {question}\nAnswer: {answer}\nTotal time: {total_time}"

    # -- --------------------------------------------------------------------------- -- #
    # -- --------------------------------------------------------------------------- -- #

    """
    Starts a terminal chat session to interact with the model.

    Args:
        tokenizer: The tokenizer for the model.
        query_pipeline: The query processing pipeline for the model.
    
    Returns:
        None
    """
    print("Welcome to the chat! Type 'exit' to end the session.")
    
    while True:
        # Gather input from the user
        in_query = input("You: ")
        
        # Exit condition
        if in_query.lower() == 'exit':
            print("Ending chat session.")
            break
        
        # Get response from the model
        response = test_model(tokenizer, query_pipeline, in_query)
        
        # Print the response
        print(f"Model: {response}")

if __name__ == "__main__":
    main()

