# Ptero CLI

This is a repository for the steganography project for *Individual Project* course on university.
<!-- TOC -->
- [Ptero CLI](#ptero-cli)
  - [Installation](#installation)
    - [Editor](#editor)
    - [Build](#build)
    - [Tests](#tests)
    - [Lint](#lint)
  - [Project structure](#project-structure)
<!-- TOC -->

## Installation

Rust is the main language used in this repository. By default, you have to install standard Rust toolchain to start working.
See [official installation page](https://www.rust-lang.org/tools/install) for more info.

### Editor 

I use the [VS Code](https://code.visualstudio.com/download) as the main editor in this project. I'd suggest to install these extensions:
* [TOML Language Support](https://marketplace.visualstudio.com/items?itemName=be5invis.toml)
* [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)
* [Rust](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
* [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)

### Build

Just `cargo build` or `cargo run` to build and run the project.

### Tests

Just `cargo t`. **Coverage is not yet set up.**

### Lint

This project ensures it is compliant with `clippy` rules. To make sure you're fine, run `cargo clippy`.

## Project structure

This project contains both binary package nad library. See [lib.rs](./src/lib.rs) to see module overview and [main.rs](./src/main.rs) the entry point for CLI binary package.