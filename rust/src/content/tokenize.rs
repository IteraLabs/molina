
use llm_models::tokenizer::LlmTokenizer;
use crate::messages::errors;
use std::path::PathBuf;

pub fn tokenize_content(text: &str) -> Result< Vec<u32>, errors::ContentError > {

    let wd_folder: String = "/Users/franciscome/git/iteralabs/molina".to_owned();
    let in_folder: &str = "/models";
    let in_subfolder: &str = "/Meta-Llama-3-8B-Instruct";
    let in_file: &str = "/tokenizer.json";
    let in_path = wd_folder.clone() + in_folder + in_subfolder + in_file;
    let path_buf: PathBuf = PathBuf::from(in_path);

    let llama_tokenizer = LlmTokenizer::new_from_tokenizer_json(&path_buf).unwrap();
    let llama_tokens = llama_tokenizer.tokenize(text);
   
    Ok(llama_tokens)

}

