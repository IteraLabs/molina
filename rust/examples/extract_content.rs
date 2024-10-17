use molina::content::extract;

use std::error::Error;
use llm_models::tokenizer::LlmTokenizer;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {

    let wd_folder: String = "/Users/franciscome/git/iteralabs/molina".to_owned();
    let in_folder: &str = "/knowledge";
    let in_subfolder: &str = "/conference_icml";
    let in_file: &str = "/alon22a.pdf";
    let in_path = wd_folder.clone() + in_folder + in_subfolder + in_file;
    
    // -- ------------------------------------------------------ CONTENT EXTRACTION -- //
    // -- ------------------------------------------------------ ------------------ -- //

    let r_extraction = extract::extract_text(&in_path);
    let raw_document = &r_extraction.unwrap();

    println!("\nDoc's page content: \n\n {:?}", raw_document[&1]);
    
    // -- ------------------------------------------------------------ TOKENIZATION -- //
    // -- ------------------------------------------------------------ ------------ -- //
    
    let in_tok = wd_folder.clone() + "/models/Meta-Llama-3-8B-Instruct/tokenizer.json";
    let path_buf = PathBuf::from(in_tok);

    let llama_tokenizer = LlmTokenizer::new_from_tokenizer_json(&path_buf)?;
    let llama_tokens = llama_tokenizer.tokenize("This is a sentence");

    println!("\nValidation: Token count: {}", llama_tokens.len());
    println!("Validation: Downloaded meta/llama3 {:?}\n", llama_tokens);

    let tokenized_doc = &raw_document[&2];
    let tokens_doc = llama_tokenizer.tokenize(tokenized_doc);

    println!("\nContent Len: {}", tokenized_doc.len());
    println!("Tokenized Content Len: {}", tokens_doc.len());
    println!("Actual Tokens: {:?}\n", tokens_doc);

    Ok(())
}
