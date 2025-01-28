# scrabble-solver

Simple scrabble-solver written in rust.

## Table of content:

- [Installation](#installation)
- [Usage](#usage)

## Installation

1. **Install [Rust](https://www.rust-lang.org/fr/tools/install)**

2. **Clone the repo:**

```sh
git clone https://github.com/Gaspard-Savoureux/scrabble-solver.git
cd scrabble-solver
```

3. **Dependencies**: Will be automatically installed when running cargo. To consult the dependencies, see [Cargo.toml](/Cargo.toml).

## Usage

**Build & Run**:

```sh
cargo run
```

man:

```sh
Usage:
    scrabble-solver [options] [INPUT] [WORDLIST]
    scrabble-solver (--help)

Options:
    -h, --help               Show this message.
    -l, --load INPUTFILE     Load existing dictionnary from a text json or bin file.
    -o, --output OUTPUTFILE  Save the dictionnary to a given file. Only support .json and .bin.
    --json-format            Format the saved file to JSON format instead of binary code.
```

**Examples**:

```sh
cargo run

OUTPUT_FILE: dict.bin
Enter your letters:
rust

RS
RT
UR
US
UT
SR
SU
ST
TR
TU
TS
RUS
RUT
URS
UST
UTS
SUR
STR
STU
TRS
TUR
RUST
RUTS
```
