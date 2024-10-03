use lopdf::{Document, Object};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Debug, Deserialize, Serialize)]

pub struct PdfText {
    // Keys are page numbers
    pub text: BTreeMap<u32, Vec<String>>,
    // Collection of errors
    pub errors: Vec<String>,
}

static IGNORE: &[&str] = &[
    "Length",
    "BBox",
    "FormType",
    "Matrix",
    "Type",
    "XObject",
    "Subtype",
    "Filter",
    "ColorSpace",
    "Width",
    "Height",
    "BitsPerComponent",
    "Length1",
    "Length2",
    "Length3",
    "PTEX.FileName",
    "PTEX.PageNumber",
    "PTEX.InfoDict",
    "FontDescriptor",
    "ExtGState",
    "MediaBox",
    "Annot",
];

fn filter_func(object_id: (u32, u16), object: &mut Object) -> Option<((u32, u16), Object)> {
    if IGNORE.contains(&object.type_name().unwrap_or_default()) {
        return None;
    }
    if let Ok(d) = object.as_dict_mut() {
        d.remove(b"Producer");
        d.remove(b"ModDate");
        d.remove(b"Creator");
        d.remove(b"ProcSet");
        d.remove(b"Procset");
        d.remove(b"XObject");
        d.remove(b"MediaBox");
        d.remove(b"Annots");
        if d.is_empty() {
            return None;
        }
    }
    Some((object_id, object.to_owned()))
}

fn load_pdf<P: AsRef<Path>>(path: P) -> Result<Document, Error> {
    Document::load_filtered(path, filter_func)
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
}

pub fn get_pdf_text(doc: &Document) -> Result<PdfText, Error> {
    let mut pdf_text: PdfText = PdfText {
        text: BTreeMap::new(),
        errors: Vec::new(),
    };

    let pages: Vec<Result<(u32, Vec<String>), Error>> = doc
        .get_pages()
        .into_iter()
        .map(
            |(page_num, page_id): (u32, (u32, u16))| -> Result<(u32, Vec<String>), Error> {
                let text = doc.extract_text(&[page_num]).map_err(|e| {
                    Error::new(
                        ErrorKind::Other,
                        format!("Failed to extract text from page {page_num} id={page_id:?}: {e:}"),
                    )
                })?;
                Ok((
                    page_num,
                    text.split('\n')
                        .map(|s| s.trim_end().to_string())
                        .collect::<Vec<String>>(),
                ))
            },
        )
        .collect();

    for page in pages {
        match page {
            Ok((page_num, lines)) => {
                pdf_text.text.insert(page_num, lines);
            }
            Err(e) => {
                pdf_text.errors.push(e.to_string());
            }
        }
    }
    Ok(pdf_text)
}

pub fn get_content<P>(
    input_path: P,
    file_pass: &str,
) -> Result<PdfText, std::io::Error>
where
    P: AsRef<Path> + Debug,
{
    let mut doc_content = load_pdf(&input_path)?;

    if doc_content.is_encrypted() {
        doc_content
            .decrypt(file_pass)
            .map_err(|_err| Error::new(ErrorKind::InvalidInput, "Failed to decrypt file"))?;
    }

    let doc_text = get_pdf_text(&doc_content)?;

    if doc_text.errors.is_empty() {
        println!("Text: {input_path:?} had 0 errors");
    } else {
        eprintln!("Text: {input_path:?} has {} errors:", doc_text.errors.len());
        for error in &doc_text.errors[..10] {
            eprintln!("{error:?}");
        }
    }


    Ok(doc_text)
}

fn pdf2text<P>(path: P, output: P, pretty: bool, password: &str) -> Result<(), Error>
where
    P: AsRef<Path> + Debug,
{
    println!("Loading the file: {path:?}");

    let mut doc = load_pdf(&path)?;

    if doc.is_encrypted() {
        doc.decrypt(password)
            .map_err(|_err| Error::new(ErrorKind::InvalidInput, "Failed to decrypt"))?;
    }

    let text = get_pdf_text(&doc)?;

    if !text.errors.is_empty() {
        eprintln!("{path:?} has {} errors:", text.errors.len());
        for error in &text.errors[..10] {
            eprintln!("{error:?}");
        }
    }

    let data = match pretty {
        true => serde_json::to_string_pretty(&text).unwrap(),
        false => serde_json::to_string(&text).unwrap(),
    };

    println!("Write {output:?}");
    let mut f = File::create(output)?;
    f.write_all(data.as_bytes())?;

    Ok(())
}

pub fn pdf_generate(in_file: &str, out_file: &str) -> Result<(), Error> {
    let start_time = Instant::now();
    let pdf_path = PathBuf::from(in_file.to_string());
    println!("pdf_path contents: {:?}", pdf_path);

    let mut output = PathBuf::from(out_file.to_string());
    output.set_extension("text");

    let pretty = true;
    let passw = "abc123";

    pdf2text(&pdf_path, &output, pretty, passw)?;

    let final_time = Instant::now().duration_since(start_time).as_secs_f64();
    println!("Executed after {:.1} seconds.", final_time);
    Ok(())
}
