## Getting Started
In this section, we will install the Ocypode interpreter and run a simple Ocypode program.

### Installation
First, let's install the Ocypode interpreter using [Cargo]:
```bash
cargo install ocypode-lang --locked
```
You can also build Ocypode from the source by running these commands:
```bash
git clone https://github.com/TheAwiteb/ocypode-lang.git
cd ocypode
cargo build --release
```
Then you can find the binary in the `target/release` directory.

You can also download the binary from the [releases page] and put it in your `PATH`.

## Basics
Each Ocypode program is a file with the `.oy` extension. The file name is the name of the program. For example, if you have a file named `hello.oy`, then the program name is `hello`.

### Main function
The main function is the entry point of the program, and it's the first function that will be executed. The main function is a function that has the name `main` and it has `argc` and `argv` parameters. The `argc` parameter is the number of arguments that are passed to the program, and the `argv` parameter is an array of strings that contains the arguments that are passed to the program. And the main function returns an integer that will be the exit code of the program and it's optional (if you don't return anything, the exit code will be `0`).
### Hello world

> **Note**: The details of functions will be explained in the [functions] section.
This just a simple program that prints `Hello world!` to the standard output.

```ocypode
~main<argc><argv>{<
    println<"Hello world!">;
>}
```
#### Output
```bash
$ ocypode hello.oy
Hello world!
```

### Comments
You can add comments to your Ocypode programs using `/` for single-line comments, and `/* */` for multi-line comments.
```ocypode
~main<argc><argv>{<
    // This is a single line comment
    /* This is
        A multi line comment 
    */
>}
```

[functions]: ../pages/functions/intro.md
[releases page]: https:://github.com/TheAwiteb/ocypode-lang/releases/latest
[Cargo]: https://doc.rust-lang.org/cargo/getting-started/installation.html