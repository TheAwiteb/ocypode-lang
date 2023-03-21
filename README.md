<div align="center">

# Ocypode

A dynamic programming language written in Rust. WIP.

[![CI](https://github.com/TheAwiteb/ocypode-lang/actions/workflows/ci.yml/badge.svg)](https://github.com/TheAwiteb/ocypode-lang/actions/workflows/ci.yml)
[![Deploy/Publish](https://github.com/TheAwiteb/ocypode-lang/actions/workflows/release-and-deploy.yml/badge.svg)](https://github.com/TheAwiteb/ocypode-lang/actions/workflows/release-and-deploy.yml)<br>
[![crate](https://img.shields.io/crates/v/ocypode-lang)](https://crates.io/crates/ocypode-lang)
[![license](https://img.shields.io/github/license/TheAwiteb/ocypode-lang)](github.com/TheAwiteb/ocypode-lang/blob/master/LICENSE)<br>
[![book](https://img.shields.io/badge/book-ocypode--lang-blue)](https://theawiteb.github.io/ocypode-lang/)

</div>

## What is Ocypode?
Ocypode is a dynamically typed language. It is an interpreted language, the interpreter written in Rust. Ocypode is object-oriented, also it has a main function that is the entry point of the program. The grammar is written in [Pest](https://pest.rs/) is a parser generator for Rust. you can find the grammar file here: [grammar.pest](https://github.com/TheAwiteb/ocypode-lang/blob/master/grammar.pest).

## Why Ocypode?
In general, Ocypode is for educational purposes. I am learning how to write a programming language, and Ocypode is the result of that. Also, Ocypode is good for how want to write a programming language in Rust.

## Documentation
Ocypode will have a book soon. For now, you can read the [tests](https://github.com/TheAwiteb/ocypode-lang/blob/master/tests) or the [grammar file](https://github.com/TheAwiteb/ocypode-lang/blob/master/grammar.pest).

## How to run Ocypode?
You can install the Ocypode interpreter/REPL by running this command:
```bash
cargo install ocypode-lang
```
Then you can run the REPL by running this command: [WIP]
```bash
ocypode
```
Or you can run a file by running this command:
```bash
ocypode <file>
```
### Or build it from the source
You can build Ocypode from the source by running these commands:
```bash
git clone https://github.com/TheAwiteb/ocypode-lang.git
cd ocypode
cargo build --release
```
Then you can find the binary in the `target/release` directory.

## How to contribute?
The contribution is welcome. You can contribute by reporting bugs, fixing bugs, adding features, or improving documentation. You can also contribute by translating the documentation into other languages.

## License
Ocypode is licensed under the [GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html) license. You can find the license file here: [LICENSE](https://github.com/TheAwiteb/ocypode-lang/blob/master/LICENSE).