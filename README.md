# molina

Welcome to the `molina` project, a synthetic research agent for local knowledge representation.

Notice here the absence of the terms `Artificial` and `Intelligence`, this is deliverate.
This is not an `artificially sweetened` project, is not a claim to be the panacea, neither 
the $n-th$ attempt to solve general let alone narrow intelligence because this is definitely
not a claim of something being intelligent. 

What is this then ?, a tool, built with an agentic approach, for you to interact with by the 
act of formulating research-related questions, which will get you responses using *only* the 
academic papers you provide as the *knowledge base*.

The name is in honor to THE greatest Mexican researcher of all times, [Mario Molina (1943-2020)](https://es.wikipedia.org/wiki/Mario_Molina_(químico)).

## Problems

Challenges of the use of Large Language Models (LLMs), as an academic research assistant, are:  

- Hallucination.
- Lack of domain-specific knowledge.
- Outdated information.
- Bias in Training Data.
- Interpretability and Response Attribution.

Those seem very closely related to what a expert human research should also avoid.

## Install

Clone the repo

```shell
git clone https://github.com/iteralabs/molina
```

Create a virtual environment and source into it.

```shell
python -m venv local_venv
source local_venv/bin/activate
```

Install packages dependencies with *pip*


```shell
pip install -r requirements.txt
```

```shell
python molina/main.py
```

## Copyright disclaimer

The purpose of this tool is not to promote or condone the unauthorized downloading or use of copyrighted materials. All users of this should only use legally obtained documents, self-archive documents and materials in their projects.

