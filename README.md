[![Coverage Status](https://coveralls.io/repos/github/baymax42/ptero-cli/badge.svg?branch=main)](https://coveralls.io/github/baymax42/ptero-cli?branch=main)
![GitHub branch checks state](https://img.shields.io/github/checks-status/baymax42/ptero-cli/main?label=status)
![GitHub](https://img.shields.io/github/license/baymax42/ptero-cli)
![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/baymax42/ptero-cli)
# Ptero CLI

This is a repository for the steganography project for *Individual Project* course on university.

It is a extended version of the alogorithm presented [here](https://link.springer.com/chapter/10.1007/978-3-319-76687-4_15). 
However, it also includes an implementation of specified algorithm. 
<!-- TOC -->
- [Ptero CLI](#ptero-cli)
  - [Installation](#installation)
    - [Editor](#editor)
    - [Build](#build)
    - [Tests](#tests)
    - [Lint](#lint)
  - [Project structure](#project-structure)
  - [Scripts](#scripts)
    - [Bitrate measurement](#bitrate-measurement)
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

## Scripts

### Bitrate measurement
There are two script in the [scripts](./scripts) directory which are used for measuring the bitrate.
The one is [bitrate_for_pivot.sh](./scripts/bitrate_for_pivot.sh) which calculates the average bitrate for
given amount of executions on random secret data, and the other one [measure_bitrate.sh](./scripts/measure_bitrate.sh) does launch measurements 
for given pivot range in parallel, 100 executions and 30 bytes secret data.

The latter one should be used to get the results. For example:
```shell script
./scripts/measure_bitrate.sh 10 40 &> result
```

It should be ran in the project root. Please note that there might be a case that given secret may not
be embedded in cover text due to the cover text capacity.

