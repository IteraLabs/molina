# Content Module

## Functionality

Extraction, pre-formatting, tokenization, splitting, injectintg and testing injection of content.

### extract_content
Extract's the PDFs content, either raw, or, filtered if such info for the parameter is included.

### split_content
sepparation into chunks, accordingly to the indicated level, to be selected from \[sub-character, character, sentence, paragraph, section, page\].

### clean_content
Perform cleaning actions as needed in the next steps (tokenizer conditions).

### tokenize_content
Transformation of the input string into vector embedings. 

### extract_plots
Similar to extract_string but only for plots and images.

### extract_tables
Similar to extract_string but only for data tables.

### extract_latex
Similar to extract_string but only for LaTeX content.    

