use std::fs;
use std::path::Path;

pub fn load_files(dir: &str) -> Vec<String> {
    let mut pdf_files = Vec::new();
    let path = Path::new(dir);

    // Recursively visit each directory and collect PDF file paths
    if path.is_dir() {

        // Use fs::read_dir to iterate through entries in the directory
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                let entry_path = entry.path();
                
                // Check if the entry is a directory or a file
                if entry_path.is_dir() {
                    // Recursion for subdirectories
                    pdf_files.extend(load_files(entry_path.to_str().unwrap()));
                } else if entry_path.extension().map(|s| s == "pdf").unwrap_or(false) {
                    // If it's a PDF file, add its path to the vector
                    pdf_files.push(entry_path.to_string_lossy().into_owned());
                }
            }
        }
    }

    pdf_files
}

