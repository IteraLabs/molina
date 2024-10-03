use molina::content::parse;

fn main() {
    let in_file_path = "input_file_1.pdf";

    /*

    let out_file_path = "output_file_1.txt";
    let pdf_content = parse::pdf_generate(in_file_path, out_file_path);
    match pdf_content {
        Ok(()) => println!("This is the Ok content {:?}",
            pdf_content),
        Err(e) => eprintln!("Operation failed: {}", e),
    }

    */

    let cont = parse::get_content(in_file_path, "abc", true);
    let first_text = &cont.unwrap();

    // Get all the keys
    println!("No. of Keys is: {:?}", &first_text.text.keys().len());

    // Return the first K, V pair
    // println!("First K,V \n{:?}", &first_text.text.first_key_value());

    // Return the last K, V pair
    // println!("Last K,V \n{:?}", &first_text.text.last_key_value());

    // for a given key, get the content
    // println!("Content {:?}", &first_text.text.get(&26));
}
