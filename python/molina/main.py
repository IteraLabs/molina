import molina
# from langchain_chroma import Chroma

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

    # -- ----------------------------------------------------------- Extract Content -- #
    # -- --------------------------------------------------------------------------- -- #
    
    result_content = molina.extract_content(input_file=in_pdf)
    print(result_content[1])

    # -- ---------------------------------------------------------- Tokenize Content -- #
    # -- --------------------------------------------------------------------------- -- #

    tokenize_result = molina.tokenize_content(result_content[1])
    print(tokenize_result) 

    # -- ------------------------------------------------------------- Split Content -- #
    # -- --------------------------------------------------------------------------- -- #
    
    # split_result = molina.split_content()
    
    # -- ------------------------------------------------------ Create Embeddings DB -- #
    # -- --------------------------------------------------------------------------- -- #
   
    #  
    
    # -- ------------------------------------------------------ Simmilarity indexing -- #
    # -- --------------------------------------------------------------------------- -- #
    
    # 

    # -- --------------------------------------------------------- Output Formatting -- #
    # -- --------------------------------------------------------------------------- -- #

if __name__ == "__main__":
    main()

