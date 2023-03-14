# Ocypode
A dynamic programming language written in Rust. WIP.

## What is Ocypode?
Ocypode is a dynamically typed language. It is a interpreted language, the interpreter written in Rust. Ocypode is object oriented, also its has a main function that is the entry point of the program. The grammar is written in [Pest](https://pest.rs/) is a parser generator for Rust. you can find the grammar file here: [grammar.pest](https://github.com/TheAwiteb/ocypode-lang/blob/master/grammar.pest).

## Why Ocypode?
In general, Ocypode is for educational purposes. I am learning how to write a programming language, and Ocypode is the result of that. Also Ocypode is good for how want to write a programming language in Rust.

## Documentation
Ocypode will have a book soon. For now you can read the [tests](https://github.com/TheAwiteb/ocypode-lang/blob/master/tests) or the [grammar file](https://github.com/TheAwiteb/ocypode-lang/blob/master/grammar.pest).

## How to run Ocypode? [WIP] ⚠️
You can install the Ocypode interpreter/REPL by running this command:
```bash
cargo install ocypode
```
Then you can run the REPL by running this command:
```bash
ocypode
```
Or you can run a file by running this command:
```bash
ocypode <file>
```
### Or build it from source
You can build Ocypode from source by running this command:
```bash
git clone https://github.com/TheAwiteb/ocypode-lang.git
cd ocypode
cargo build --release
```
Then you can find the binary in the `target/release` directory.

## How to contribute?
The contribution are welcome. You can contribute by reporting bugs, fixing bugs, adding features, or improving the documentation. You can also contribute by translating the documentation to other languages.

## License
Ocypode is licensed under the [GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html) license. You can find the license file here: [LICENSE](https://github.com/TheAwiteb/ocypode-lang/blob/master/LICENSE).