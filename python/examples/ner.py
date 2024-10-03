import spacy
from molina import get_content

# Load the English NLP model
nlp = spacy.load("en_core_web_sm")
# Sample text
text = "Apple is looking at buying U.K. startup for $1 billion. Tim Cook is the CEO of Apple."

# Process the text with spaCy
doc = nlp(text)

# Print named entities
for ent in doc.ents:
    print(f"Text: {ent.text}, Label: {ent.label_}")

get_content("input_file_1.pdf")
