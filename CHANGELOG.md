# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- The interpreter of the language. Now you can run the code.
- Built-in functions
    - `format` To format a string
    - `print`/`println` To print in the stdout (including formatting)
    - `len` To get the length
    - `push` To push an element to the end of an array
    - `pop` To pop an element from the end of an array
    - `input` To get input from the user
### Changed
- Now the array can contain a expressions, not only a value

## [0.1.0] - 2023-3-14
### Added
Just the parser of the language, and the AST. also the grammar and tests of the parser.