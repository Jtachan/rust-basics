# rust-basics

Here are contained basic Rust projects, which main objective is just to train the basics of the language.
Each project is either:

- A standalone script (which executable can be created with `rustc`).
- A cargo crate, for those projects that are a little bit more complicated.

Alongside the projects, in this document is presented a cheat-sheet over some basic rust usages.

## Index

- [**Theory**](#rust-basic-theory)
- [**Projects**](#uploaded-projects)
    - [Scripts](#scripts)
    - [Crates](#crates)

## Rust basic theory

### Formatting

**Single files (scrits)**
```
rustfmt FILE_PATH.rs
```

**Cargo**
```
fmt
```

> Note: The formatting will also return linting messages.

### Compiling

**Single files**
```
rustc FILE_PATH.rs
```

**Cargo**

- `cargo build`: Create an executable at _target/debug/CRATE-NAME_.
- `cargo run`: Creates the previous executable and runs it.
- `cargo check`: Sanity checks over the code without compiling.
- `cargo build --release`: Create an executable at _target/release/CRATE-NAME_. The code compiles with optimizations for the finla executable to run faster.

## Uploaded projects

Here are listed all projects that are uploaded in the repository.
They all link to the main file, that is either the script RS file for the scripts or the 'src/main.rs' file at the crates.

### Scripts

- [Temperature converter](./scripts/temperature_converter.rs)

### Crates

- [Number guessing game](./crates/guessing_game/src/main.rs)
