use molina::content::extract;
// use tokenizers::tokenizer::{Tokenizer, Result};
// use tokenizers::models::bpe::BPE;

fn main() {
    let wd_folder: String = "/Users/franciscome/git/iteralabs/molina".to_owned();
    let in_folder: &str = "/knowledge";
    let in_subfolder: &str = "/conference_icml";
    let in_file: &str = "/alon22a.pdf";
    let in_path = wd_folder + in_folder + in_subfolder + in_file;
    
    // -- ------------------------------------------------------ CONTENT EXTRACTION -- //
    // -- ------------------------------------------------------ ------------------ -- //

    let r_extraction = extract::extract_text(&in_path);
    let raw_document = &r_extraction.unwrap();

    println!("\nDoc's page content: \n\n {:?}", raw_document[&1]);
    
    // -- ------------------------------------------------------------ TOKENIZATION -- //
    // -- ------------------------------------------------------------ ------------ -- //

    // let tokenized_doc = &raw_document[&2];
    // let tokens = tokenizer.encode(tokenized_doc, false)?;

    //println!("Token Count: {}", tokens.len());
    //println!("Tokens: {:?}", tokens);

    // Print the raw contents, structured as indicated in the docs
    // https://docs.rs/lopdf/latest/lopdf/struct.Document.html
    // let obj_document: Vec<_> = raw_document.objects.clone().into_values().collect();
    // println!("The objects that compose the PDF: {:?}", obj_document);

    // Get all the keys
    // println!("No. of Keys is: {:?}", &first_text.text.keys().len());

    // Return the first K, V pair
    // println!("First K,V \n{:?}", &first_text.text.first_key_value());

    // Return the last K, V pair
    //println!("Last K,V \n{:?}", &first_text.text.last_key_value());

    // for a given key, get the content
    // println!("Content {:?}", &first_text.text.get(&26));
}
