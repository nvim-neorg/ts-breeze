# Massive Code Collections Parsed Fast

The `breeze` library allows developers to parse large collections of files with any `tree-sitter`
grammar in a multithreaded fashion.

## Treesitter Versioning

Tree-sitter currently does not have a tagged release that supports converting to and from raw pointers to underlying treesitter data structures.
As a result, this crate temporarily requires tree-sitter from a specific commit - `d0029a15273e526925a764033e9b7f18f96a7ce5`.
