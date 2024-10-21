
from langchain_community.embeddings import GPT4AllEmbeddings

gguf_model_name = "all-MiniLM-L6-v2.gguf2.f16.gguf"
gpt4all_kwargs = {'allow_download': 'True'}

embedding_model = GPT4AllEmbeddings(model_name=gguf_model_name, gpt4all_kwargs=gpt4all_kwargs)

