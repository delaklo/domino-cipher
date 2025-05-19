# domino-cipher

A simple cipher tool written in pure Rust. It encodes and decodes messages using a 7x7 Polybius-square-like cipher grid, mapping each character to a domino-style coordinate. Th encryption is fully reversible and the tool supports secret-key-based grid permutation for custom encoding.

This project works entirely with the Rust standard library.

## What it does

domino-cipher takes input text and transforms each character into a pair of numbers that correspond to its row and column in a 7x7 grid. This is similar to a Polybius square but with full support for a broader symbol set, like punctuation and special characters.

The grid contains 49 characters in total:

ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,!?-_()#*&

When encrypting a message, each character is located in the grid and replaced with its `[row|column]` coordinate. When decrypting, the process is reversed using the same grid layout.

You can provide a secret key via the `--key` option. The key permutes the grid so that its characters are placed first in order (without duplicates), followed by the remaining characters in default order.

## How to use

You can build and run the program with Cargo.

To encrypt a message:
```
cargo run -- --encrypt "HELLO WORLD" --key SECRET
```

To decrypt a message back into plaintext:

```
cargo run -- --decrypt "[1|3],[0|1],[2|0],[2|0],[2|3],[6|6],[3|1],[2|3],[0|3],[2|0],[1|0]" --key SECRET
```
To see the active Polybius grid based on a secret key:
```
cargo run -- --key SECRET --print-grid
```
If no key is provided, the default grid order will be used. 

This project is designed for experimentation, puzzles, or encoding basic information in a playful way. 
