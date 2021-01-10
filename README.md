

# Ptero CLI ![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/baymax42/ptero-cli)

---

**Ptero** is a CLI text steganography tool, meant to be used in the social media like *Facebook*. It offers a way to encode and decode data.

It implements the algorithm presented [here](https://link.springer.com/chapter/10.1007/978-3-319-76687-4_15). 
However, it also includes an extended implementation of specified algorithm, called __ELUV__. See `help` of the tool for more info on the usage.

The tool was tested in *Facebook's Messenger* and *Twitter* - both methods were working correctly. You can also read more about the method [here](https://github.com/baymax42/ptero-cli/wiki/Methods).

---

<!-- TOC -->
- [Ptero CLI](#ptero-cli)
  - [Development](#development)
    - [Editor](#editor)
    - [Build](#build)
    - [Tests and coverage](#tests-and-coverage)
    - [Lint](#lint)
  - [Project structure](#project-structure)
  - [Scripts](#scripts)
    - [Bitrate measurement](#bitrate-measurement)
<!-- TOC -->

## Development   [![Coverage Status](https://coveralls.io/repos/github/baymax42/ptero-cli/badge.svg?branch=main)](https://coveralls.io/github/baymax42/ptero-cli?branch=main) ![GitHub branch checks state](https://img.shields.io/github/checks-status/baymax42/ptero-cli/main?label=status) ![GitHub](https://img.shields.io/github/license/baymax42/ptero-cli)

Rust is the main language used in this repository. By default, you have to install standard Rust toolchain to start working.
See [official installation page](https://www.rust-lang.org/tools/install) for more info.

### Editor 

I use the [VS Code](https://code.visualstudio.com/download) as the main editor in this project. I'd suggest to install these extensions:
* [TOML Language Support](https://marketplace.visualstudio.com/items?itemName=be5invis.toml)
* [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)
* [Rust](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
* [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)

### Build

If you want to just build the artifact run:
```shell
cargo build
```

To run the binary through the `cargo` you can issue:
```shell
# Example with argument
cargo run -- -vv encode -c some_cover_text -d secret
```
### Tests and coverage

To run all the tests:
```shell
cargo test
```

Coverage checking is done through separate package `cargo-tarpaulin` - make sure to install it if you want to. To run it locally just:
```shell
cargo tarpaulin -v
```

### Lint

This project ensures it is compliant with `clippy` rules. To make sure you're fine, run:
```shell
cargo clippy
```

## Project structure

This project contains both binary package nad library. See [lib.rs](./src/lib.rs) to see module overview and [main.rs](./src/bin/main.rs) the entry point for CLI binary package.


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

