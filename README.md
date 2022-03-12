# enLSP

## Description
An english language server protocol aimed at autocompletion, grammar, syntax highlighting and more

## Installation

To get this LSP working (at whatever the current stage is) you need to install the following dependencies:
- Cargo

### dependencies:

Debian based:
```
sudo apt install cargo
```

Arch based:
```
sudo pacman -S cargo 
```

Fedora based:
```bash
sudo dnf install cargo
```

### install:

after having that dependencies installed, you can install the LSP by running the following command:
```bash
cargo install enlsp (currently not viable btw, please build from source)
```
or build from source:
```bash
git clone https://github.com/wylited/enLSP.git
cd enlsp
cargo build --release
``` 
or debug mode with `cargo build --debug` or just `cargo build`

(currently the built binary will be found in the `target/debug` folder or `target/release` folder)

## Configuration
some configuration file will be made later and an explanation of how to configure it will be described.


## Feature Roadmap
- [] basic LSP
- [] basic autocompletion
- [] basic syntax highlighting
    - [] parts of speech
        - [] nouns
        - [] pronouns
        - [] verbs
        - [] adjective
        - [] adverb
        - [] preposition
        - [] conjuction
        - [] interjection
        - [] article
    - [] tenses
        - [] simple
            - [] present
            - [] past
            - [] future
        - [] continuous
            - [] present
            - [] past
            - [] future
        - [] perfect
            - [] present
            - [] past
            - [] future
        - [] perfect continous
            - [] present
            - [] past
            - [] future
- [] basic grammar
- [] advanced autocomplete. (intelligent autocomplete using context)
- [] advanced grammar. (intelligent grammar using context)
- [] 

## Troubleshooting
idk message me at wylited#1010 on discord if you want help or want to help.