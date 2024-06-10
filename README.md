# Blask-Project

Blask is an instruction set architecture specification designed for 16-bit handheld gaming consoles.
The aim of the project is to build a platform that empowers people to create retro-style 16-bit games and to learn about computers.
For this purpose, we designed Blask to be as simple as possible while still being capable enough to design interesting projects for it.

## Why?

Computers have become much more powerful but also much harder to study and understand than 25 years ago.
Modern computers have become too sophisticated to be used as case studies for beginner students who want to learn the low-level intricacies of computers.
There is a need for a platform that empowers beginners to study assembly language, computer architecture, embedded programming, emulators, assemblers, debuggers, logic design etc..

## Structure

This repository has many crates.

### Lexer

The lexer is in the `blex` crate.

### Parser

The parser is divided into multiple steps and therefore into multiple crates.
The first step is to transform our list of tokens into **Lossless Syntax Tree (LST)** nodes. 
This is a separate step so that we can feed this into a formatter to format the code in its textual form.

This first step is taking care of in the `blarse` crate and constructs an **LST** using the type in the `blalst` crate.

The second step is constructing an **Abstract Syntax Tree (AST)** which is done in the `blaast` crate.

### Assembler

The core of the assembler is in the `blas` crate.

If you wish to use as a **CLI** utility, this is implemented in the `assembler` crate:

```sh
cargo run --bin assembler -- input_file -o output_file
```

### Emulator

The emulator is contained in the `emulator` crate.

For more information:

```sh
cargo run --bin emulator --help
```

## Testing

You can run all tests for everything in every crate by running:

```sh
cargo test
```

Or you can specify the crate you want to test this way:

```sh
cargo test crate
```
