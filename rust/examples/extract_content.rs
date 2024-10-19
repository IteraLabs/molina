use molina::content::extract;
use molina::data::loader;
use std::error::Error;
use llm_models::tokenizer::LlmTokenizer;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {

    let wd_folder: String = "/Users/franciscome/git/iteralabs/molina".to_owned();
    let in_folder: &str = "/knowledge";
    let in_subfolder: &str = "/conference_icml";
    let in_file: &str = "/mao24c.pdf";
    let in_path = wd_folder.clone() + in_folder + in_subfolder + in_file;

    // -------------------------------------------------------------- FILES LOADING -- //
    // -------------------------------------------------------------- ------------- -- //
    
    //let v_files = loader::load_files(&in_path);
    //println!("v_files is: {:?}", &v_files);

    // -- ------------------------------------------------------ CONTENT EXTRACTION -- //
    // -- ------------------------------------------------------ ------------------ -- //

    //let in_file: &str = "/1-s2.0-S0032063323002052-main.pdf";
    let r_extraction = extract::extract_text(&in_path);
    println!("r_extraction has: {:?}", &r_extraction);
    
    let raw_document = &r_extraction?;
    println!("\nDoc's page content: \n\n {:?}", raw_document[&4]);
    
    // -- ------------------------------------------------------------ TOKENIZATION -- //
    // -- ------------------------------------------------------------ ------------ -- //
    
    let in_tok = wd_folder.clone() + "/models/Meta-Llama-3-8B-Instruct/tokenizer.json";
    let path_buf = PathBuf::from(in_tok);
    let llama_tokenizer = LlmTokenizer::new_from_tokenizer_json(&path_buf)?;
    
    // let llama_tokens = llama_tokenizer.tokenize("This is a sentence");
    //println!("\nValidation: Token count: {}", llama_tokens.len());
    //println!("Validation: Downloaded meta/llama3 {:?}\n", llama_tokens);

    let tokenized_doc = &raw_document[&2];
    let tokens_doc = llama_tokenizer.tokenize(tokenized_doc);

    println!("\nContent Len: {}", tokenized_doc.len());
    println!("Tokenized Content Len: {}", tokens_doc.len());
    println!("Actual Tokens: {:?}\n", tokens_doc);

    Ok(())
}
