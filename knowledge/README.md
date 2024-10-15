# Knowledge Base

Only Journal Articles with OpenAccess rights.

## Folder structure

It should exist a `knowledge` folder, then each of its sub-folder is either a source (e.g. one per journal) and within 
each of the sub-folder also files should be present (any naming convention, as log it is consistent).

```bash
molina/
└─ knowledge/
   ├── conference_icml/
   │    ├── alon22a.pdf 
   │    ├── ...
   │    └── zrnic24a.pdf
   ├── journal_fb/
   │    ├── fbloc-02-00014.pdf
   │    ├── ...
   │    └── fbloc-07-1455070.pdf
   └── journal_jifmim/
       ├── 1-s2.0-S1042443117302858-main.pdf
       ├── ...
       └── 1-s2.0-S1544612324012200-main.pdf
```
## Source Catalog

A file, as agnostic from any programming language as possible, to express or represent the folder structure and meta data about the file's contents. 

```json
"publisher": {
    "elsevier": {
        "journal" : {
            "jifmim": {
                "abv": "jifmim",
                "name": "Journal of International Financial Markets, Institutions and Money",
                "isOpenAccess": false, 
                "hasOpenAccess": true,
            },
            "pss": {
                "abv": "pss",
                "name": "Planetary and Space Science",
                "isOpenAccess": false,
                "hasOpenAccess": true,
            },
        },
    },
    "frontiers": {
        "journal": {
            "fb": {
                "abv": "fb",
                "name": "Frontiers in Blockchain",
                "isOpenAccess": true,
                "HasOpenAccess": true,
            },
        },
    },
    "icml": {
        "conference": {
            "icml": {
                "abv": "icml",
                "name": "International Conference on Artificial Intelligence",
                "isOpenAccess": true,
                "hasOpenAccess": true,
            }
        }
    }
}
```

## Downloaded content

### Publisher: [Elsevier](https://www.sciencedirect.com)

- Some are Open Access.
- [copyright](https://www.elsevier.com/open-access)
- Publications:
    - **PSS** (Journal): Planetary and Space Science.
    - **JIFMIM** (Journal): Journal of International Financial Markets, Instutions and Money.

### Publisher : [Frontiers](https://www.frontiersin.org/articles)

- All is Open Access.
- [copyright](https://www.frontiersin.org/about/open-access)
- Publications:
    - **FiB** (Journal): Frontiers in Blockchain

### Publisher: [ICML](https://icml.cc/) 

- All is Open Access
- [copyright](https://icml.cc/FAQ/Copyright)
- Publications:
    - **PMLR** (Conference Proceedings): 2024:2021

