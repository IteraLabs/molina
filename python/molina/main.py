from molina import extract_content
import torch
import transformers

def main():

    wd_folder = "/Users/franciscome/git/iteralabs/molina"
    in_folder = "/knowledge"
    in_subfolder = "/conference_icml"
    in_file = "/basu24a.pdf"
    in_pdf = wd_folder + in_folder + in_subfolder + in_file
    # in_tokenizer = wd_folder + "/models/Meta-Llama-3-8B-Instruct/"

    result_content = extract_content(input_file=in_pdf)

    print(f"the resulting keys were: {result_content.keys()}")
    print(result_content[1])

    # -- --------------------------------------------------------------------------- -- #

if __name__ == "__main__":
    main()

